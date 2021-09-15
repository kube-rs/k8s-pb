/// MetricIdentifier identifies a metric by name and, optionally, selector
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricIdentifier {
    /// name is the name of the given metric
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// selector represents the label selector that could be used to select
    /// this metric, and will generally just be the selector passed in to
    /// the query used to fetch this metric.
    /// When left blank, only the metric's Name will be used to gather metrics.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}
/// MetricListOptions is used to select metrics by their label selectors
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricListOptions {
    /// A selector to restrict the list of returned objects by their labels.
    /// Defaults to everything.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub label_selector: ::core::option::Option<::prost::alloc::string::String>,
    /// A selector to restrict the list of returned metrics by their labels
    /// +optional
    #[prost(string, optional, tag="2")]
    pub metric_label_selector: ::core::option::Option<::prost::alloc::string::String>,
}
/// MetricValue is the metric value for some object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValue {
    /// a reference to the described object
    #[prost(message, optional, tag="1")]
    pub described_object: ::core::option::Option<super::super::super::super::super::api::core::v1::ObjectReference>,
    #[prost(message, optional, tag="2")]
    pub metric: ::core::option::Option<MetricIdentifier>,
    /// indicates the time at which the metrics were produced
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// indicates the window ([Timestamp-Window, Timestamp]) from
    /// which these metrics were calculated, when returning rate
    /// metrics calculated from cumulative metrics (or zero for
    /// non-calculated instantaneous metrics).
    #[prost(int64, optional, tag="4")]
    pub window_seconds: ::core::option::Option<i64>,
    /// the value of the metric for this
    #[prost(message, optional, tag="5")]
    pub value: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// MetricValueList is a list of values for a given metric for some set of objects
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValueList {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// the value of the metric across the described objects
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<MetricValue>,
}
