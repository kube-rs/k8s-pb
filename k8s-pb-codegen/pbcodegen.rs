use prost::Message;
use prost_types::{FileDescriptorProto, FileDescriptorSet};
#[macro_use]
extern crate log;
use anyhow::{Context, Result};
use pbcodegen::*;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    Ok(fs::read_to_string(&path).with_context(|| format!("read {}", path.as_ref().display()))?)
}

fn main() -> Result<()> {
    std::env::set_var("RUST_LOG", "pbcodegen=info");
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

    info!("loading api-resources");
    let apif: String = read_file(root.join("openapi/api-resources.json"))?;
    let apis: Vec<GenApiGroupResources> =
        serde_json::from_str(&apif).with_context(|| "parse api-resources.json".to_string())?;

    let buf =
        std::fs::read(root.join("protos.fds")).with_context(|| "read protos.fds".to_string())?;
    let fds = FileDescriptorSet::decode(&*buf).unwrap(); // pulls in proto::Message

    // NB: FDS fields: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-types/src/protobuf.rs#L1-L7
    // FDS usage: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-build/src/lib.rs#L765-L825
    for f in fds.file {
        if let Some(pkg) = f.package {
            let pkgpath = Path::new("./out").join(format!("{}.rs", pkg));
            // match the pkg name to a key in api-resources.json
            info!("generating {}", pkg);
            if let Some(apikey) = pkgname_to_api_key(&pkg) {
                let mut codegen = vec![];
                // find the corresponding apiGroupVersion
                if let Some(apigr) = apis.iter().find(|ag| ag.apiGroupVersion == apikey) {
                    for dp in f.message_type {
                        if let Some(name) = dp.name {
                            let children: Vec<String> =
                                apigr.resources.iter().map(|gr| gr.kind.clone()).collect();
                            info!("  - found {:?} in {}", children, apikey);
                            if name.ends_with("List") {
                                continue;
                            }
                            // find the inner resource with matching kind name
                            if let Some(api) = apigr.resources.iter().find(|gr| name == gr.kind) {
                                codegen.push(api.generics());
                            } else {
                                debug!("  - no-generics for {}/{} (not in {})", pkg, name, apikey);
                            }
                        }
                    }
                } else {
                    warn!("skipped: {} (didn't find {})", pkg, apikey);
                }
                let generics = codegen.join("\n");
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&pkgpath)?;
                file.write(generics.as_bytes())?;
            }
        }
    }

    // Generate code in `src/` by reading files in `OUT_DIR`?
    // Need to create `mod.rs` file for each level based on the filename, and write generated code in correct file.
    Ok(())
}
