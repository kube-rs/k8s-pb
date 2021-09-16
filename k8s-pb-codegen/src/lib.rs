// codify structs in api-resource.json
// this is the root struct (we have a vector of them)
#[derive(serde::Deserialize)]
pub struct GenApiGroupResources {
    pub apiGroupVersion: String,
    pub resources: Vec<GenApiResource>,
}
// main resource struct
#[derive(serde::Deserialize)]
pub struct GenApiResource {
    /// plural name
    pub name: String,
    #[serde(default)]
    pub namespaced: bool,
    pub subresource: bool,
    /// apigroup/ver
    pub apiGroupVersion: String,
    /// apigroup
    pub group: String,
    /// ver
    pub version: String,
    pub kind: String,
    /// expected module path :: delimited
    pub rust: String,
    /// allowed verbs
    pub verbs: Vec<String>,
    pub scopedVerbs: ScopedVerbs,
    /// vec of found root paths
    ///
    /// "/apis/apps/v1/controllerrevisions",
    /// "/apis/apps/v1/namespaces/{namespace}/controllerrevisions",
    /// "/apis/apps/v1/namespaces/{namespace}/controllerrevisions/{name}"
    pub paths: Vec<String>,
}
#[derive(serde::Deserialize)]
pub struct ScopedVerbs {
    #[serde(default)]
    pub all: Vec<String>,
    #[serde(default)]
    pub namespaced: Vec<String>,
}

impl GenApiResource {
    pub fn generics(&self) -> String {
        // TODO: use codegen crate here ?
        format!("// generics for {} {}", self.name, self.apiGroupVersion)
    }
}

pub fn pkgname_to_api_key(pkg: &str) -> Option<String> {
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
