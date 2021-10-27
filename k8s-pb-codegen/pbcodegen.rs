#[macro_use]
extern crate log;
use anyhow::{Context, Result};
use std::{fs, path::Path};

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

    // Generate code in `src/` by reading files in `OUT_DIR`?
    // Need to create `mod.rs` file for each level based on the filename, and write generated code in correct file.
    Ok(())
}
