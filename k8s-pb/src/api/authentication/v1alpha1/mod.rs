/// SelfSubjectReview contains the user information that the kube-apiserver has about the user making this request.
/// When using impersonation, users will receive the user info of the user being impersonated.  If impersonation or
/// request header authentication is used, any extra keys will have their case ignored and returned as lowercase.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfSubjectReview {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// Status is filled in by the server with the user attributes.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<SelfSubjectReviewStatus>,
}
/// SelfSubjectReviewStatus is filled by the kube-apiserver and sent back to a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfSubjectReviewStatus {
    /// User attributes of the user making this request.
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub user_info: ::core::option::Option<super::v1::UserInfo>,
}

impl crate::Resource for SelfSubjectReview {
    const API_VERSION: &'static str = "authentication.k8s.io/v1alpha1";
    const GROUP: &'static str = "authentication.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "SelfSubjectReview";
    const URL_PATH_SEGMENT: &'static str = "selfsubjectreviews";
    type Scope = crate::ClusterResourceScope;
}
impl crate::HasMetadata for SelfSubjectReview {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasStatus for SelfSubjectReview {
    type Status = crate::api::authentication::v1alpha1::SelfSubjectReviewStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}

