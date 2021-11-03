#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    /// Name of the resource used in URL.
    pub name: String,
    /// True if the resource is namespaced. May still have `all` verbs like `list`.
    pub namespaced: bool,
    pub api_version: String,
    pub group: String,
    pub version: String,
    pub kind: String,
    /// Protobuf type path.
    pub proto: String,
    /// Rust type path.
    pub rust: String,
    /// Metadata type.
    pub metadata: Option<String>,
    /// Spec type if any.
    pub spec: Option<String>,
    /// Status type if any.
    pub status: Option<String>,
    /// Condition type if the resource has `.status.conditions`.
    pub condition: Option<String>,
    /// Verbs grouped by scope. `all` or `namespaced`.
    pub scoped_verbs: ScopedVerbs,
    /// All paths associated with this resource.
    pub paths: Vec<String>,
    // TODO `Option<Vec<Subresource>>`
    /// Any subresources.
    pub subresources: Vec<Subresource>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopedVerbs {
    pub all: Option<Vec<String>>,
    /// Namespaced actions.
    pub namespaced: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subresource {
    /// Name of the subresouce used in URL.
    pub name: String,
    /// Verbs grouped by scope. `all` or `namespaced`.
    pub scoped_verbs: ScopedVerbs,
    /// All paths associated with this subresource.
    pub paths: Vec<String>,
}
