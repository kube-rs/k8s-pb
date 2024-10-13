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
        .btree_map(&["."])
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
            info!("building pkg: {package_name}");
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
        /// The scope of a [`Resource`].
        pub trait ResourceScope {}

        /// Indicates that a [`Resource`] is cluster-scoped.
        pub struct ClusterResourceScope {}
        impl ResourceScope for ClusterResourceScope {}

        /// Indicates that a [`Resource`] is namespace-scoped.
        pub struct NamespaceResourceScope {}
        impl ResourceScope for NamespaceResourceScope {}

        /// Indicates that a [`Resource`] is neither cluster-scoped nor namespace-scoped.
        pub struct SubResourceScope {}
        impl ResourceScope for SubResourceScope {}

        /// A trait applied to all Kubernetes resources.
        pub trait Resource {
            /// The API version of the resource. This is a composite of [`Resource::GROUP`] and [`Resource::VERSION`] (eg `"apiextensions.k8s.io/v1beta1"`)
            /// or just the version for resources without a group (eg `"v1"`).
            ///
            /// This is the string used in the `apiVersion` field of the resource's serialized form.
            const API_VERSION: &'static str;

            /// The group of the resource, or the empty string if the resource doesn't have a group.
            const GROUP: &'static str;

            /// The kind of the resource.
            ///
            /// This is the string used in the `kind` field of the resource's serialized form.
            const KIND: &'static str;
            /// The version of the resource.
            const VERSION: &'static str;

            /// The URL path segment used to construct URLs related to this resource.
            ///
            /// For cluster- and namespaced-scoped resources, this is the plural name of the resource that is followed by the resource name.
            /// For example, [`api::core::v1::Pod`](crate::api::core::v1::Pod)'s value is `"pods"` and its URLs look like `.../pods/{name}`.
            ///
            /// For subresources, this is the subresource name that comes after the parent resource's name.
            /// For example, [`api::authentication::v1::TokenRequest`](crate::api::authentication::v1::TokenRequest)'s value is `"token"`,
            /// and its URLs look like `.../serviceaccounts/{name}/token`.
            const URL_PATH_SEGMENT: &'static str;

            /// Indicates whether the resource is namespace-scoped or cluster-scoped or a subresource.
            ///
            /// If you need to restrict some generic code to resources of a specific scope, use this associated type to create a bound on the generic.
            /// For example, `fn foo<T: k8s_openapi::Resource<Scope = k8s_openapi::ClusterResourceScope>>() { }` can only be called with cluster-scoped resources.
            type Scope: ResourceScope;
        }

        /// A trait applied to all Kubernetes resources that have Metadata
        pub trait Metadata: Resource {
            type Ty;
            fn metadata(&self) -> Option<&Self::Ty>;
            fn metadata_mut(&mut self) -> Option<&mut Self::Ty>;
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
    let scope = if resource.namespaced {
        format_ident!("NamespaceResourceScope")
    } else {
        format_ident!("ClusterResourceScope")
    };
    let tokens = quote! {
        impl crate::Resource for #type_name {
            const API_VERSION: &'static str = #api_version;
            const GROUP: &'static str = #group;
            const VERSION: &'static str = #version;
            const KIND: &'static str = #kind;
            const URL_PATH_SEGMENT: &'static str = #name;
            type Scope = crate::#scope;
        }
    };
    let tokens = if let Some(metadata) = &resource.metadata {
        let metadata = metadata.split("::").map(|m| format_ident!("{}", m));
        quote! {
            #tokens

            impl crate::Metadata for #type_name {
                type Ty = crate::#(#metadata)::*;

                fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
                    self.metadata.as_ref()
                }
                fn metadata_mut(&mut self) -> Option<&mut<Self as crate::Metadata>::Ty> {
                    self.metadata.as_mut()
                }
            }
        }
    } else {
        tokens
    };
    let tokens = if let Some(spec) = &resource.spec {
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

    writeln!(pkg_rs).unwrap();
    writeln!(pkg_rs, "{}", &tokens).unwrap();
}
