use prost::Message;
use prost_types::FileDescriptorSet;
#[macro_use]
extern crate log;
use anyhow::{Context, Result};
use std::{collections::HashMap, fs, path::Path};

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
    prost_build::Config::new()
        // should probably switch to this
        //.btree_map(&["."])
        .out_dir("./out")
        .compile_protos(protos.as_slice(), &["protos/"])?;

    info!("loading json");
    let apif: String = read_file(root.join("openapi/transformed.json"))?;
    let resources: HashMap<String, Resource> =
        serde_json::from_str(&apif).with_context(|| "parse transformed.json".to_string())?;

    let buf =
        std::fs::read(root.join("protos.fds")).with_context(|| "read protos.fds".to_string())?;
    let fds = FileDescriptorSet::decode(&*buf).unwrap(); // pulls in proto::Message

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
        }
    }

    // Generate code in `src/` by reading files in `OUT_DIR`?
    // Need to create `mod.rs` file for each level based on the filename, and write generated code in correct file.
    Ok(())
}
