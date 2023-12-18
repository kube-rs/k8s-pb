use std::{
    collections::{BTreeSet, HashMap},
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
};

use prost::Message;
use prost_types::FileDescriptorSet;
#[macro_use]
extern crate log;
use anyhow::{Context, Result};
use quote::{format_ident, quote};

use k8s_pb_codegen::Resource;

fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    fs::read_to_string(&path).with_context(|| format!("read {}", path.as_ref().display()))
}

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "k8s_pb_codegen=info");
    env_logger::init();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    info!("parsing protos.list in {}", root.display());

    let proto_file: String = read_file(root.join("protos.list"))?;
    let protos = proto_file.lines().collect::<Vec<&str>>();
    debug!("protos: {:?}", protos);

    info!("building protos");
    let target_dir = root.join("../k8s-pb/src");
    let tmp_dir = root.join("./tmp");
    let descriptor_file = tmp_dir.join("protos.fds");
    prost_build::Config::new()
        // should probably switch to this
        //.btree_map(&["."])
        .out_dir(&tmp_dir)
        .file_descriptor_set_path(&descriptor_file)
        .compile_protos(protos.as_slice(), &["protos/"])?;

    info!("loading json");
    let apif: String = read_file(root.join("openapi/transformed.json"))?;
    let resources: HashMap<String, Resource> =
        serde_json::from_str(&apif).with_context(|| "parse transformed.json".to_string())?;

    let buf = std::fs::read(descriptor_file).with_context(|| "read protos.fds".to_string())?;
    let fds = FileDescriptorSet::decode(&*buf).unwrap(); // pulls in proto::Message

    // Map of module path to the names of child modules.
    let mut module_tree: HashMap<String, BTreeSet<String>> = HashMap::new();
    // Generated packages
    let mut pkgs: Vec<String> = Vec::new();
    // NB: FDS fields: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-types/src/protobuf.rs#L1-L7
    // FDS usage: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-build/src/lib.rs#L765-L825
    for f in fds.file {
        if let Some(package_name) = f.package {
            println!(
                "got package name: {}, looking in {}",
                package_name,
                tmp_dir.display()
            );
            if package_name.contains(".schema") {
                continue;
            }
            let mut pkg_rs = OpenOptions::new()
                .append(true)
                .open(tmp_dir.join(format!("{}.rs", package_name)))
                .unwrap();

            for msg in f.message_type {
                let message_name = match msg.name {
                    Some(ref name) => name,
                    None => continue,
                };
                let path = format!("{}.{}", package_name, message_name);
                if let Some(resource) = resources.get(&path) {
                    append_trait_impl(&mut pkg_rs, message_name, resource);
                }
            }

            let mut parts = package_name.split('.').collect::<Vec<_>>();
            while !parts.is_empty() {
                let module = parts.pop().unwrap();
                let parent = parts.join("/");
                module_tree
                    .entry(parent)
                    .or_insert_with(BTreeSet::new)
                    .insert(module.to_owned());
            }
            pkgs.push(package_name);
        }
    }

    for (k, mods) in module_tree {
        let dst = target_dir.join(if k.is_empty() {
            "lib.rs".to_owned()
        } else {
            format!("{}/mod.rs", k)
        });
        std::fs::create_dir_all(dst.parent().unwrap())?;
        let lines = mods
            .into_iter()
            .map(|m| format!("pub mod {};", m))
            .collect::<Vec<_>>();
        std::fs::write(dst, lines.join("\n") + "\n")?;
    }

    for pkg in pkgs {
        let src = tmp_dir.join(format!("{}.rs", &pkg));
        let dst = target_dir.join(format!("{}/mod.rs", pkg.replace('.', "/")));
        std::fs::create_dir_all(dst.parent().unwrap())?;
        std::fs::rename(src, dst)?;
    }

    let mut lib_rs = OpenOptions::new()
        .append(true)
        .open(target_dir.join("lib.rs"))
        .unwrap();
    append_trait_def(&mut lib_rs);

    Ok(())
}

fn append_trait_def(lib_rs: &mut File) {
    let tokens = quote! {
        pub trait Resource {
            const API_VERSION: &'static str;
            const GROUP: &'static str;
            const VERSION: &'static str;
            const KIND: &'static str;
            const NAME: &'static str;
        }

        pub trait HasMetadata {
            type Metadata;
            fn metadata(&self) -> Option<&Self::Metadata>;
            fn metadata_mut(&mut self) -> Option<&mut Self::Metadata>;
        }
        pub trait HasSpec {
            type Spec;
            fn spec(&self) -> Option<&Self::Spec>;
            fn spec_mut(&mut self) -> Option<&mut Self::Spec>;
        }
        pub trait HasStatus {
            type Status;
            fn status(&self) -> Option<&Self::Status>;
            fn status_mut(&mut self) -> Option<&mut Self::Status>;
        }
        pub trait HasConditions {
            type Condition;
            fn conditions(&self) -> Option<&[Self::Condition]>;
            fn conditions_mut(&mut self) -> Option<&mut Vec<Self::Condition>>;
        }
    };
    let tokens = rustfmt(tokens).unwrap();
    writeln!(lib_rs).unwrap();
    writeln!(lib_rs, "{}", &tokens).unwrap();
}

fn append_trait_impl(pkg_rs: &mut File, message_name: &str, resource: &Resource) {
    use heck::ToUpperCamelCase;
    // Convert to match prost
    let type_name = message_name.to_upper_camel_case();
    let type_name = format_ident!("{}", type_name);

    let api_version = &resource.api_version;
    let group = &resource.group;
    let kind = &resource.kind;
    let version = &resource.version;
    let name = &resource.name;
    let tokens = quote! {
        impl crate::Resource for #type_name {
            const API_VERSION: &'static str = #api_version;
            const GROUP: &'static str = #group;
            const VERSION: &'static str = #version;
            const KIND: &'static str = #kind;
            const NAME: &'static str = #name;
        }
    };
    let tokens = if let Some(metadata) = &resource.metadata {
        let metadata = metadata.split("::").map(|m| format_ident!("{}", m));
        quote! {
            #tokens

            impl crate::HasMetadata for #type_name {
                type Metadata = crate::#(#metadata)::*;

                fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
                    self.metadata.as_ref()
                }
                fn metadata_mut(&mut self) -> Option<&mut<Self as crate::HasMetadata>::Metadata> {
                    self.metadata.as_mut()
                }
            }
        }
    } else {
        tokens
    };
    let tokens = if let Some(spec) = &resource.spec {
        // HACK Prost changes message name `APIService` to `ApiService`, and `transformed.json` uses the original name.
        let spec = spec
            .replace("APIService", "ApiService")
            .replace("CSIDriver", "CsiDriver")
            .replace("ClusterCIDR", "ClusterCidr")
            .replace("IPAddress", "IpAddress")
            .replace("CSINode", "CsiNode");
        let spec = spec.split("::").map(|m| format_ident!("{}", m));
        quote! {
            #tokens

            impl crate::HasSpec for #type_name {
                type Spec = crate::#(#spec)::*;

                fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
                    self.spec.as_ref()
                }
                fn spec_mut(&mut self) -> Option<&mut<Self as crate::HasSpec>::Spec> {
                    self.spec.as_mut()
                }
            }
        }
    } else {
        tokens
    };
    let tokens = if let Some(status) = &resource.status {
        // HACK Prost changes message name `APIService` to `ApiService`, and `transformed.json` uses the original name.
        let status = status.replace("APIService", "ApiService");
        let status = status.split("::").map(|m| format_ident!("{}", m));
        quote! {
            #tokens

            impl crate::HasStatus for #type_name {
                type Status = crate::#(#status)::*;

                fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
                    self.status.as_ref()
                }
                fn status_mut(&mut self) -> Option<&mut<Self as crate::HasStatus>::Status> {
                    self.status.as_mut()
                }
            }
        }
    } else {
        tokens
    };

    let tokens = if let Some(condition) = &resource.condition {
        // HACK Prost changes message name `APIService` to `ApiService`, and `transformed.json` uses the original name.
        let condition = condition.replace("APIService", "ApiService");
        let condition = condition.split("::").map(|m| format_ident!("{}", m));
        quote! {
            #tokens

            impl crate::HasConditions for #type_name {
                type Condition = crate::#(#condition)::*;

                fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
                    self.status.as_ref().map(|s| s.conditions.as_slice())
                }
                fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
                    self.status.as_mut().and_then(|s| Some(s.conditions.as_mut()))
                }
            }
        }
    } else {
        tokens
    };

    let tokens = rustfmt(tokens).unwrap();
    writeln!(pkg_rs).unwrap();
    writeln!(pkg_rs, "{}", &tokens).unwrap();
}

// Run `rustfmt` on an in-memory string
fn rustfmt(text: impl std::fmt::Display) -> Result<String, Box<dyn std::error::Error>> {
    let mut rustfmt = std::process::Command::new("rustfmt");
    rustfmt
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped());
    let mut rustfmt = rustfmt.spawn()?;
    write!(rustfmt.stdin.take().unwrap(), "{}", text)?;
    let output = rustfmt.wait_with_output()?;
    let stdout = String::from_utf8(output.stdout)?;
    Ok(stdout)
}
