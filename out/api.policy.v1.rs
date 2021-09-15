/// Eviction evicts a pod from its node subject to certain policies and safety constraints.
/// This is a subresource of Pod.  A request to cause such an eviction is
/// created by POSTing to .../pods/<pod name>/evictions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Eviction {
    /// ObjectMeta describes the pod that is being evicted.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// DeleteOptions may be provided
    /// +optional
    #[prost(message, optional, tag="2")]
    pub delete_options: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::DeleteOptions>,
}
/// PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDisruptionBudget {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the PodDisruptionBudget.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<PodDisruptionBudgetSpec>,
    /// Most recently observed status of the PodDisruptionBudget.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<PodDisruptionBudgetStatus>,
}
/// PodDisruptionBudgetList is a collection of PodDisruptionBudgets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDisruptionBudgetList {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is a list of PodDisruptionBudgets
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<PodDisruptionBudget>,
}
/// PodDisruptionBudgetSpec is a description of a PodDisruptionBudget.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDisruptionBudgetSpec {
    /// An eviction is allowed if at least "minAvailable" pods selected by
    /// "selector" will still be available after the eviction, i.e. even in the
    /// absence of the evicted pod.  So for example you can prevent all voluntary
    /// evictions by specifying "100%".
    /// +optional
    #[prost(message, optional, tag="1")]
    pub min_available: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// Label query over pods whose evictions are managed by the disruption
    /// budget.
    /// A null selector will match no pods, while an empty ({}) selector will select
    /// all pods within the namespace.
    /// +patchStrategy=replace
    /// +optional
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// An eviction is allowed if at most "maxUnavailable" pods selected by
    /// "selector" are unavailable after the eviction, i.e. even in absence of
    /// the evicted pod. For example, one can prevent all voluntary evictions
    /// by specifying 0. This is a mutually exclusive setting with "minAvailable".
    /// +optional
    #[prost(message, optional, tag="3")]
    pub max_unavailable: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
}
/// PodDisruptionBudgetStatus represents information about the status of a
/// PodDisruptionBudget. Status may trail the actual state of a system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodDisruptionBudgetStatus {
    /// Most recent generation observed when updating this PDB status. DisruptionsAllowed and other
    /// status information is valid only if observedGeneration equals to PDB's object generation.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// DisruptedPods contains information about pods whose eviction was
    /// processed by the API server eviction subresource handler but has not
    /// yet been observed by the PodDisruptionBudget controller.
    /// A pod will be in this map from the time when the API server processed the
    /// eviction request to the time when the pod is seen by PDB controller
    /// as having been marked for deletion (or after a timeout). The key in the map is the name of the pod
    /// and the value is the time when the API server processed the eviction request. If
    /// the deletion didn't occur and a pod is still there it will be removed from
    /// the list automatically by PodDisruptionBudget controller after some time.
    /// If everything goes smooth this map should be empty for the most of the time.
    /// Large number of entries in the map may indicate problems with pod deletions.
    /// +optional
    #[prost(map="string, message", tag="2")]
    pub disrupted_pods: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Number of pod disruptions that are currently allowed.
    #[prost(int32, optional, tag="3")]
    pub disruptions_allowed: ::core::option::Option<i32>,
    /// current number of healthy pods
    #[prost(int32, optional, tag="4")]
    pub current_healthy: ::core::option::Option<i32>,
    /// minimum desired number of healthy pods
    #[prost(int32, optional, tag="5")]
    pub desired_healthy: ::core::option::Option<i32>,
    /// total number of pods counted by this disruption budget
    #[prost(int32, optional, tag="6")]
    pub expected_pods: ::core::option::Option<i32>,
    /// Conditions contain conditions for PDB. The disruption controller sets the
    /// DisruptionAllowed condition. The following are known values for the reason field
    /// (additional reasons could be added in the future):
    /// - SyncFailed: The controller encountered an error and wasn't able to compute
    ///               the number of allowed disruptions. Therefore no disruptions are
    ///               allowed and the status of the condition will be False.
    /// - InsufficientPods: The number of pods are either at or below the number
    ///                     required by the PodDisruptionBudget. No disruptions are
    ///                     allowed and the status of the condition will be False.
    /// - SufficientPods: There are more pods than required by the PodDisruptionBudget.
    ///                   The condition will be True, and the number of allowed
    ///                   disruptions are provided by the disruptionsAllowed property.
    ///
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag="7")]
    pub conditions: ::prost::alloc::vec::Vec<super::super::super::apimachinery::pkg::apis::meta::v1::Condition>,
}
// NB: no-generics for api.policy.v1/Eviction (not in policy/v1)
// TODO generics for poddisruptionbudgets policy/v1
// NB: no-generics for api.policy.v1/PodDisruptionBudgetSpec (not in policy/v1)
// NB: no-generics for api.policy.v1/PodDisruptionBudgetStatus (not in policy/v1)