use prost_types::{FileDescriptorProto, FileDescriptorSet};
use prost::Message;

fn main() -> std::io::Result<()> {
    let protos = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/protos.list"))
        .lines()
        .collect::<Vec<&str>>();

    prost_build::Config::new()
        // should probably switch to this
        //.btree_map(&["."])
        .out_dir("./out")
        .compile_protos(protos.as_slice(), &["protos/"])?;

    let apis = std::fs::read_to_string("./openapi/api-resources.json")?;

    let buf = std::fs::read("./protos.fds").unwrap();
    let fds = FileDescriptorSet::decode(&*buf).unwrap(); // pulls in proto::Message

    // NB: FDS fields: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-types/src/protobuf.rs#L1-L7
    // FDS usage: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-build/src/lib.rs#L765-L825
    for f in fds.file {
        use std::io::Write;
        if let Some(pkg) = f.package {
            let pkgpath = std::path::Path::new("./out").join(format!("{}.rs", pkg));
            let generics = format!("// TODO genericsfor {}\n", pkg);
            let mut file = std::fs::OpenOptions::new().write(true).append(true).open(&pkgpath)?;
            file.write(generics.as_bytes())?;
        }
    }

    // Generate code in `src/` by reading files in `OUT_DIR`?
    // Need to create `mod.rs` file for each level based on the filename, and write generated code in correct file.
    Ok(())
}
