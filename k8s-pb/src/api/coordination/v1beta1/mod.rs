/// Lease defines a lease concept.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lease {
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the Lease.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<LeaseSpec>,
}
/// LeaseList is a list of Lease objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of schema objects.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Lease>,
}
/// LeaseSpec is a specification of a Lease.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseSpec {
    /// holderIdentity contains the identity of the holder of a current lease.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub holder_identity: ::core::option::Option<::prost::alloc::string::String>,
    /// leaseDurationSeconds is a duration that candidates for a lease need
    /// to wait to force acquire it. This is measure against time of last
    /// observed RenewTime.
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub lease_duration_seconds: ::core::option::Option<i32>,
    /// acquireTime is a time when the current lease was acquired.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub acquire_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime>,
    /// renewTime is a time when the current holder of a lease has last
    /// updated the lease.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub renew_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime>,
    /// leaseTransitions is the number of transitions of a lease between
    /// holders.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub lease_transitions: ::core::option::Option<i32>,
}
