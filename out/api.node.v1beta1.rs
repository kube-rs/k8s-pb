/// Overhead structure represents the resource overhead associated with running a pod.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Overhead {
    /// PodFixed represents the fixed resource overhead associated with running a pod.
    /// +optional
    #[prost(map="string, message", tag="1")]
    pub pod_fixed: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// RuntimeClass defines a class of container runtime supported in the cluster.
/// The RuntimeClass is used to determine which container runtime is used to run
/// all containers in a pod. RuntimeClasses are (currently) manually defined by a
/// user or cluster provisioner, and referenced in the PodSpec. The Kubelet is
/// responsible for resolving the RuntimeClassName reference before running the
/// pod.  For more details, see
/// https://git.k8s.io/enhancements/keps/sig-node/585-runtime-class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeClass {
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Handler specifies the underlying runtime and configuration that the CRI
    /// implementation will use to handle pods of this class. The possible values
    /// are specific to the node & CRI configuration.  It is assumed that all
    /// handlers are available on every node, and handlers of the same name are
    /// equivalent on every node.
    /// For example, a handler called "runc" might specify that the runc OCI
    /// runtime (using native Linux containers) will be used to run the containers
    /// in a pod.
    /// The Handler must be lowercase, conform to the DNS Label (RFC 1123) requirements,
    /// and is immutable.
    #[prost(string, optional, tag="2")]
    pub handler: ::core::option::Option<::prost::alloc::string::String>,
    /// Overhead represents the resource overhead associated with running a pod for a
    /// given RuntimeClass. For more details, see
    /// https://git.k8s.io/enhancements/keps/sig-node/688-pod-overhead/README.md
    /// This field is beta-level as of Kubernetes v1.18, and is only honored by servers that enable the PodOverhead feature.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub overhead: ::core::option::Option<Overhead>,
    /// Scheduling holds the scheduling constraints to ensure that pods running
    /// with this RuntimeClass are scheduled to nodes that support it.
    /// If scheduling is nil, this RuntimeClass is assumed to be supported by all
    /// nodes.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub scheduling: ::core::option::Option<Scheduling>,
}
/// RuntimeClassList is a list of RuntimeClass objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeClassList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of schema objects.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<RuntimeClass>,
}
/// Scheduling specifies the scheduling constraints for nodes supporting a
/// RuntimeClass.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scheduling {
    /// nodeSelector lists labels that must be present on nodes that support this
    /// RuntimeClass. Pods using this RuntimeClass can only be scheduled to a
    /// node matched by this selector. The RuntimeClass nodeSelector is merged
    /// with a pod's existing nodeSelector. Any conflicts will cause the pod to
    /// be rejected in admission.
    /// +optional
    /// +mapType=atomic
    #[prost(map="string, string", tag="1")]
    pub node_selector: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// tolerations are appended (excluding duplicates) to pods running with this
    /// RuntimeClass during admission, effectively unioning the set of nodes
    /// tolerated by the pod and the RuntimeClass.
    /// +optional
    /// +listType=atomic
    #[prost(message, repeated, tag="2")]
    pub tolerations: ::prost::alloc::vec::Vec<super::super::core::v1::Toleration>,
}
// TODO genericsfor api.node.v1beta1
