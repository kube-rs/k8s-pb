/// ImageReview checks if the set of images in a pod are allowed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageReview {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec holds information about the pod being evaluated
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ImageReviewSpec>,
    /// Status is filled in by the backend and indicates whether the pod should be allowed.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<ImageReviewStatus>,
}
/// ImageReviewContainerSpec is a description of a container within the pod creation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageReviewContainerSpec {
    /// This can be in the form image:tag or image@SHA:012345679abcdef.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
}
/// ImageReviewSpec is a description of the pod creation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageReviewSpec {
    /// Containers is a list of a subset of the information in each container of the Pod being created.
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub containers: ::prost::alloc::vec::Vec<ImageReviewContainerSpec>,
    /// Annotations is a list of key-value pairs extracted from the Pod's annotations.
    /// It only includes keys which match the pattern `*.image-policy.k8s.io/*`.
    /// It is up to each webhook backend to determine how to interpret these annotations, if at all.
    /// +optional
    #[prost(map="string, string", tag="2")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Namespace is the namespace the pod is being created in.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
}
/// ImageReviewStatus is the result of the review for the pod creation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageReviewStatus {
    /// Allowed indicates that all images were allowed to be run.
    #[prost(bool, optional, tag="1")]
    pub allowed: ::core::option::Option<bool>,
    /// Reason should be empty unless Allowed is false in which case it
    /// may contain a short description of what is wrong.  Kubernetes
    /// may truncate excessively long errors when displaying to the user.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// AuditAnnotations will be added to the attributes object of the
    /// admission controller request using 'AddAnnotation'.  The keys should
    /// be prefix-less (i.e., the admission controller will add an
    /// appropriate prefix).
    /// +optional
    #[prost(map="string, string", tag="3")]
    pub audit_annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
// didn't find imagepolicy/v1alpha1