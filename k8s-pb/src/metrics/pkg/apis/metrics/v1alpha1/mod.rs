/// ContainerMetrics sets resource usage metrics of a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerMetrics {
    /// Container name corresponding to the one from pod.spec.containers.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The memory usage is the memory working set.
    #[prost(map="string, message", tag="2")]
    pub usage: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// NodeMetrics sets resource usage metrics of a node.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeMetrics {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// The following fields define time interval from which metrics were
    /// collected from the interval [Timestamp-Window, Timestamp].
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    #[prost(message, optional, tag="3")]
    pub window: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Duration>,
    /// The memory usage is the memory working set.
    #[prost(map="string, message", tag="4")]
    pub usage: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// NodeMetricsList is a list of NodeMetrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeMetricsList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of node metrics.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<NodeMetrics>,
}
/// PodMetrics sets resource usage metrics of a pod.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodMetrics {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// The following fields define time interval from which metrics were
    /// collected from the interval [Timestamp-Window, Timestamp].
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    #[prost(message, optional, tag="3")]
    pub window: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Duration>,
    /// Metrics for all containers are collected within the same time window.
    #[prost(message, repeated, tag="4")]
    pub containers: ::prost::alloc::vec::Vec<ContainerMetrics>,
}
/// PodMetricsList is a list of PodMetrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodMetricsList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// List of pod metrics.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<PodMetrics>,
}
