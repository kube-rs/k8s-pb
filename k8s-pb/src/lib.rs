pub mod api;
pub mod apiextensions_apiserver;
pub mod apimachinery;
pub mod kube_aggregator;
pub mod metrics;

#[doc = r" The scope of a [`Resource`]."]
pub trait ResourceScope {}
#[doc = r" Indicates that a [`Resource`] is cluster-scoped."]
pub struct ClusterResourceScope {}
impl ResourceScope for ClusterResourceScope {}
#[doc = r" Indicates that a [`Resource`] is namespace-scoped."]
pub struct NamespaceResourceScope {}
impl ResourceScope for NamespaceResourceScope {}
#[doc = r" Indicates that a [`Resource`] is neither cluster-scoped nor namespace-scoped."]
pub struct SubResourceScope {}
impl ResourceScope for SubResourceScope {}
#[doc = r" A trait applied to all Kubernetes resources."]
pub trait Resource {
    #[doc = r#" The API version of the resource. This is a composite of [`Resource::GROUP`] and [`Resource::VERSION`] (eg `"apiextensions.k8s.io/v1beta1"`)"#]
    #[doc = r#" or just the version for resources without a group (eg `"v1"`)."#]
    #[doc = r""]
    #[doc = r" This is the string used in the `apiVersion` field of the resource's serialized form."]
    const API_VERSION: &'static str;
    #[doc = r" The group of the resource, or the empty string if the resource doesn't have a group."]
    const GROUP: &'static str;
    #[doc = r" The kind of the resource."]
    #[doc = r""]
    #[doc = r" This is the string used in the `kind` field of the resource's serialized form."]
    const KIND: &'static str;
    #[doc = r" The version of the resource."]
    const VERSION: &'static str;
    #[doc = r" The URL path segment used to construct URLs related to this resource."]
    #[doc = r""]
    #[doc = r" For cluster- and namespaced-scoped resources, this is the plural name of the resource that is followed by the resource name."]
    #[doc = r#" For example, [`api::core::v1::Pod`](crate::api::core::v1::Pod)'s value is `"pods"` and its URLs look like `.../pods/{name}`."#]
    #[doc = r""]
    #[doc = r" For subresources, this is the subresource name that comes after the parent resource's name."]
    #[doc = r#" For example, [`api::authentication::v1::TokenRequest`](crate::api::authentication::v1::TokenRequest)'s value is `"token"`,"#]
    #[doc = r" and its URLs look like `.../serviceaccounts/{name}/token`."]
    const URL_PATH_SEGMENT: &'static str;
    #[doc = r" Indicates whether the resource is namespace-scoped or cluster-scoped or a subresource."]
    #[doc = r""]
    #[doc = r" If you need to restrict some generic code to resources of a specific scope, use this associated type to create a bound on the generic."]
    #[doc = r" For example, `fn foo<T: k8s_openapi::Resource<Scope = k8s_openapi::ClusterResourceScope>>() { }` can only be called with cluster-scoped resources."]
    type Scope: ResourceScope;
}
#[doc = r" A trait applied to all Kubernetes resources that have Metadata"]
pub trait Metadata: Resource {
    type Ty;
    fn metadata(&self) -> &Self::Ty;
    fn metadata_mut(&mut self) -> &mut Self::Ty;
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
