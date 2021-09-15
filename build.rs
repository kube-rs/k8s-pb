use prost_types::{FileDescriptorProto, FileDescriptorSet};
use prost::Message;

// codify structs in api-resource.json
// this is the root struct (we have a vector of them)
#[derive(serde::Deserialize)]
struct GenApiGroupResources {
    apiGroupVersion: String,
    resources: Vec<GenApiResource>
}
// main resource struct
#[derive(serde::Deserialize)]
struct GenApiResource {
    /// plural name
    name: String,
    #[serde(default)]
    namespaced: bool,
    subresource: bool,
    /// apigroup/ver
    apiGroupVersion: String,
    /// apigroup
    group: String,
    /// ver
    version: String,
    kind: String,
    /// expected module path :: delimited
    rust: String,
    /// allowed verbs
    verbs: Vec<String>,
    scopedVerbs: ScopedVerbs,
    /// vec of found root paths
    ///
    /// "/apis/apps/v1/controllerrevisions",
    /// "/apis/apps/v1/namespaces/{namespace}/controllerrevisions",
    /// "/apis/apps/v1/namespaces/{namespace}/controllerrevisions/{name}"
    paths: Vec<String>,
}
#[derive(serde::Deserialize)]
struct ScopedVerbs {
    #[serde(default)]
    all: Vec<String>,
    #[serde(default)]
    namespaced: Vec<String>,
}

impl GenApiResource {
    fn generics(&self) -> String {
        format!("// TODO generics for {} {}", self.name, self.apiGroupVersion)
    }
}

fn pkgname_to_api_key(pkg: &str) -> Option<String> {
    // TODO: this function is dumb. we probably need to have a better key in the root object than apiGroupVersion
    // otherwise we'd have to match up weird paths like api.storage.v1 -> storage.k8s.io/v1
    if let Some((catpth, ver)) = pkg.rsplit_once(".") {
        if let Some((category, pth)) = catpth.split_once(".") {
            match category {
                "api" => Some(format!("{}/{}", pth, ver)),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}


fn main() -> std::io::Result<()> {
    let protos = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/protos.list"))
        .lines()
        .collect::<Vec<&str>>();

    prost_build::Config::new()
        // should probably switch to this
        //.btree_map(&["."])
        .out_dir("./out")
        .compile_protos(protos.as_slice(), &["protos/"])?;

    let apif = std::fs::read_to_string("./openapi/api-resources.json")?;
    let apis: Vec<GenApiGroupResources> = serde_json::from_str(&apif)?;

    let buf = std::fs::read("./protos.fds").unwrap();
    let fds = FileDescriptorSet::decode(&*buf).unwrap(); // pulls in proto::Message

    // NB: FDS fields: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-types/src/protobuf.rs#L1-L7
    // FDS usage: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-build/src/lib.rs#L765-L825
    for f in fds.file {
        use std::io::Write;
        if let Some(pkg) = f.package {
            let pkgpath = std::path::Path::new("./out").join(format!("{}.rs", pkg));
            // match the pkg name to a key in api-resources.json
            if let Some(apikey) = pkgname_to_api_key(&pkg) {
                let mut codegen = vec![];
                // find the corresponding apiGroupVersion
                if let Some(apigr) = apis.iter().find(|ag| ag.apiGroupVersion == apikey) {
                    for dp in f.message_type {
                        if let Some(name) = dp.name {
                            if name.ends_with("List") {
                                continue;
                            }
                            // find the inner resource with matching kind name
                            if let Some(api) = apigr.resources.iter().find(|gr| name == gr.kind) {
                                codegen.push(api.generics());
                            } else {
                                codegen.push(format!("// NB: no-generics for {}/{} (not in {})", pkg, name, apikey));
                            }
                        }
                    }
                } else {
                    codegen.push(format!("// didn't find {}", apikey));
                }
                let generics = codegen.join("\n");
                let mut file = std::fs::OpenOptions::new().write(true).append(true).open(&pkgpath)?;
                file.write(generics.as_bytes())?;

            }
        }
    }

    // Generate code in `src/` by reading files in `OUT_DIR`?
    // Need to create `mod.rs` file for each level based on the filename, and write generated code in correct file.
    Ok(())
}
