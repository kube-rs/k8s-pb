/// Carp is a collection of containers, used as either input (create, update) or as output (list, get).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Carp {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the carp.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<CarpSpec>,
    /// Most recently observed status of the carp.
    /// This data may not be up to date.
    /// Populated by the system.
    /// Read-only.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<CarpStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarpCondition {
    /// Type is the type of the condition.
    /// Currently only Ready.
    /// More info: <http://kubernetes.io/docs/user-guide/carp-states#carp-conditions>
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status is the status of the condition.
    /// Can be True, False, Unknown.
    /// More info: <http://kubernetes.io/docs/user-guide/carp-states#carp-conditions>
    #[prost(string, optional, tag = "2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time we probed the condition.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub last_probe_time: ::core::option::Option<super::super::meta::v1::Time>,
    /// Last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub last_transition_time: ::core::option::Option<super::super::meta::v1::Time>,
    /// Unique, one-word, CamelCase reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// Human-readable message indicating details about last transition.
    /// +optional
    #[prost(string, optional, tag = "6")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// CarpList is a list of Carps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarpList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::meta::v1::ListMeta>,
    /// List of carps.
    /// More info: <http://kubernetes.io/docs/user-guide/carps>
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Carp>,
}
/// CarpSpec is a description of a carp
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarpSpec {
    /// Restart policy for all containers within the carp.
    /// One of Always, OnFailure, Never.
    /// Default to Always.
    /// More info: <http://kubernetes.io/docs/user-guide/carp-states#restartpolicy>
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub restart_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// Optional duration in seconds the carp needs to terminate gracefully. May be decreased in delete request.
    /// Value must be non-negative integer. The value zero indicates delete immediately.
    /// If this value is nil, the default grace period will be used instead.
    /// The grace period is the duration in seconds after the processes running in the carp are sent
    /// a termination signal and the time when the processes are forcibly halted with a kill signal.
    /// Set this value longer than the expected cleanup time for your process.
    /// Defaults to 30 seconds.
    /// +optional
    #[prost(int64, optional, tag = "4")]
    pub termination_grace_period_seconds: ::core::option::Option<i64>,
    /// Optional duration in seconds the carp may be active on the node relative to
    /// StartTime before the system will actively try to mark it failed and kill associated containers.
    /// Value must be a positive integer.
    /// +optional
    #[prost(int64, optional, tag = "5")]
    pub active_deadline_seconds: ::core::option::Option<i64>,
    /// NodeSelector is a selector which must be true for the carp to fit on a node.
    /// Selector which must match a node's labels for the carp to be scheduled on that node.
    /// More info: <http://kubernetes.io/docs/user-guide/node-selection/README>
    /// +optional
    #[prost(map = "string, string", tag = "7")]
    pub node_selector: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// ServiceAccountName is the name of the ServiceAccount to use to run this carp.
    /// More info: <https://kubernetes.io/docs/concepts/security/service-accounts/>
    /// +optional
    #[prost(string, optional, tag = "8")]
    pub service_account_name: ::core::option::Option<::prost::alloc::string::String>,
    /// DeprecatedServiceAccount is a depreciated alias for ServiceAccountName.
    /// Deprecated: Use serviceAccountName instead.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(string, optional, tag = "9")]
    pub service_account: ::core::option::Option<::prost::alloc::string::String>,
    /// NodeName is a request to schedule this carp onto a specific node. If it is non-empty,
    /// the scheduler simply schedules this carp onto that node, assuming that it fits resource
    /// requirements.
    /// +optional
    #[prost(string, optional, tag = "10")]
    pub node_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Host networking requested for this carp. Use the host's network namespace.
    /// If this option is set, the ports that will be used must be specified.
    /// Default to false.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(bool, optional, tag = "11")]
    pub host_network: ::core::option::Option<bool>,
    /// Use the host's pid namespace.
    /// Optional: Default to false.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(bool, optional, tag = "12")]
    pub host_pid: ::core::option::Option<bool>,
    /// Use the host's ipc namespace.
    /// Optional: Default to false.
    /// +k8s:conversion-gen=false
    /// +optional
    #[prost(bool, optional, tag = "13")]
    pub host_ipc: ::core::option::Option<bool>,
    /// Specifies the hostname of the Carp
    /// If not specified, the carp's hostname will be set to a system-defined value.
    /// +optional
    #[prost(string, optional, tag = "16")]
    pub hostname: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified, the fully qualified Carp hostname will be "<hostname>.<subdomain>.<carp namespace>.svc.<cluster domain>".
    /// If not specified, the carp will not have a domainname at all.
    /// +optional
    #[prost(string, optional, tag = "17")]
    pub subdomain: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified, the carp will be dispatched by specified scheduler.
    /// If not specified, the carp will be dispatched by default scheduler.
    /// +optional
    #[prost(string, optional, tag = "19")]
    pub schedulername: ::core::option::Option<::prost::alloc::string::String>,
}
/// CarpStatus represents information about the status of a carp. Status may trail the actual
/// state of a system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarpStatus {
    /// Current condition of the carp.
    /// More info: <http://kubernetes.io/docs/user-guide/carp-states#carp-phase>
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub phase: ::core::option::Option<::prost::alloc::string::String>,
    /// Current service state of carp.
    /// More info: <http://kubernetes.io/docs/user-guide/carp-states#carp-conditions>
    /// +optional
    #[prost(message, repeated, tag = "2")]
    pub conditions: ::prost::alloc::vec::Vec<CarpCondition>,
    /// A human readable message indicating details about why the carp is in this condition.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
    /// A brief CamelCase message indicating details about why the carp is in this state.
    /// e.g. 'DiskPressure'
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// IP address of the host to which the carp is assigned. Empty if not yet scheduled.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub host_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// IP address allocated to the carp. Routable at least within the cluster.
    /// Empty if not yet allocated.
    /// +optional
    #[prost(string, optional, tag = "6")]
    pub carp_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet.
    /// This is before the Kubelet pulled the container image(s) for the carp.
    /// +optional
    #[prost(message, optional, tag = "7")]
    pub start_time: ::core::option::Option<super::super::meta::v1::Time>,
}
