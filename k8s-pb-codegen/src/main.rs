use prost::Message;
use prost_types::FileDescriptorSet;
#[macro_use]
extern crate log;
use anyhow::{Context, Result};
use std::{
    collections::{BTreeSet, HashMap},
    fs,
    path::Path,
};

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
    prost_build::Config::new()
        // should probably switch to this
        //.btree_map(&["."])
        .out_dir(&tmp_dir)
        .compile_protos(protos.as_slice(), &["protos/"])?;

    info!("loading json");
    let apif: String = read_file(root.join("openapi/transformed.json"))?;
    let resources: HashMap<String, Resource> =
        serde_json::from_str(&apif).with_context(|| "parse transformed.json".to_string())?;

    let buf =
        std::fs::read(root.join("protos.fds")).with_context(|| "read protos.fds".to_string())?;
    let fds = FileDescriptorSet::decode(&*buf).unwrap(); // pulls in proto::Message

    // Map of module path to the names of child modules.
    let mut module_tree: HashMap<String, BTreeSet<String>> = HashMap::new();
    // Generated packages
    let mut pkgs: Vec<String> = Vec::new();
    // NB: FDS fields: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-types/src/protobuf.rs#L1-L7
    // FDS usage: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-build/src/lib.rs#L765-L825
    for f in fds.file {
        if let Some(pkg) = f.package {
            for m in f.message_type {
                if let Some(name) = m.name {
                    let path = format!("{}.{}", pkg, name);
                    if let Some(resource) = resources.get(&path) {
                        info!("generating resource {}", path);
                        debug!("{:?}", resource);
                    }
                }
            }

            let mut parts = pkg.split('.').collect::<Vec<_>>();
            while !parts.is_empty() {
                let module = parts.pop().unwrap();
                let parent = parts.join("/");
                module_tree
                    .entry(parent)
                    .or_insert_with(BTreeSet::new)
                    .insert(module.to_owned());
            }
            pkgs.push(pkg);
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

    Ok(())
}
