/// AllowedCSIDriver represents a single inline CSI Driver that is allowed to be used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedCsiDriver {
    /// Name is the registered name of the CSI driver
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// AllowedFlexVolume represents a single Flexvolume that is allowed to be used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedFlexVolume {
    /// driver is the name of the Flexvolume driver.
    #[prost(string, optional, tag="1")]
    pub driver: ::core::option::Option<::prost::alloc::string::String>,
}
/// AllowedHostPath defines the host volume conditions that will be enabled by a policy
/// for pods to use. It requires the path prefix to be defined.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedHostPath {
    /// pathPrefix is the path prefix that the host volume must match.
    /// It does not support `*`.
    /// Trailing slashes are trimmed when validating the path prefix with a host path.
    ///
    /// Examples:
    /// `/foo` would allow `/foo`, `/foo/` and `/foo/bar`
    /// `/foo` would not allow `/food` or `/etc/foo`
    #[prost(string, optional, tag="1")]
    pub path_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// when set to true, will allow host volumes matching the pathPrefix only if all volume mounts are readOnly.
    /// +optional
    #[prost(bool, optional, tag="2")]
    pub read_only: ::core::option::Option<bool>,
}
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
/// FSGroupStrategyOptions defines the strategy type and options used to create the strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FsGroupStrategyOptions {
    /// rule is the strategy that will dictate what FSGroup is used in the SecurityContext.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub rule: ::core::option::Option<::prost::alloc::string::String>,
    /// ranges are the allowed ranges of fs groups.  If you would like to force a single
    /// fs group then supply a single range with the same start and end. Required for MustRunAs.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub ranges: ::prost::alloc::vec::Vec<IdRange>,
}
/// HostPortRange defines a range of host ports that will be enabled by a policy
/// for pods to use.  It requires both the start and end to be defined.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostPortRange {
    /// min is the start of the range, inclusive.
    #[prost(int32, optional, tag="1")]
    pub min: ::core::option::Option<i32>,
    /// max is the end of the range, inclusive.
    #[prost(int32, optional, tag="2")]
    pub max: ::core::option::Option<i32>,
}
/// IDRange provides a min/max of an allowed range of IDs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdRange {
    /// min is the start of the range, inclusive.
    #[prost(int64, optional, tag="1")]
    pub min: ::core::option::Option<i64>,
    /// max is the end of the range, inclusive.
    #[prost(int64, optional, tag="2")]
    pub max: ::core::option::Option<i64>,
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
    /// items list individual PodDisruptionBudget objects
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
    /// A null selector selects no pods.
    /// An empty selector ({}) also selects no pods, which differs from standard behavior of selecting all pods.
    /// In policy/v1, an empty selector will select all pods in the namespace.
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
/// PodSecurityPolicy governs the ability to make requests that affect the Security Context
/// that will be applied to a pod and container.
/// Deprecated in 1.21.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSecurityPolicy {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec defines the policy enforced.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<PodSecurityPolicySpec>,
}
/// PodSecurityPolicyList is a list of PodSecurityPolicy objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSecurityPolicyList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is a list of schema objects.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<PodSecurityPolicy>,
}
/// PodSecurityPolicySpec defines the policy enforced.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSecurityPolicySpec {
    /// privileged determines if a pod can request to be run as privileged.
    /// +optional
    #[prost(bool, optional, tag="1")]
    pub privileged: ::core::option::Option<bool>,
    /// defaultAddCapabilities is the default set of capabilities that will be added to the container
    /// unless the pod spec specifically drops the capability.  You may not list a capability in both
    /// defaultAddCapabilities and requiredDropCapabilities. Capabilities added here are implicitly
    /// allowed, and need not be included in the allowedCapabilities list.
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub default_add_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// requiredDropCapabilities are the capabilities that will be dropped from the container.  These
    /// are required to be dropped and cannot be added.
    /// +optional
    #[prost(string, repeated, tag="3")]
    pub required_drop_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// allowedCapabilities is a list of capabilities that can be requested to add to the container.
    /// Capabilities in this field may be added at the pod author's discretion.
    /// You must not list a capability in both allowedCapabilities and requiredDropCapabilities.
    /// +optional
    #[prost(string, repeated, tag="4")]
    pub allowed_capabilities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// volumes is an allowlist of volume plugins. Empty indicates that
    /// no volumes may be used. To allow all volumes you may use '*'.
    /// +optional
    #[prost(string, repeated, tag="5")]
    pub volumes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// hostNetwork determines if the policy allows the use of HostNetwork in the pod spec.
    /// +optional
    #[prost(bool, optional, tag="6")]
    pub host_network: ::core::option::Option<bool>,
    /// hostPorts determines which host port ranges are allowed to be exposed.
    /// +optional
    #[prost(message, repeated, tag="7")]
    pub host_ports: ::prost::alloc::vec::Vec<HostPortRange>,
    /// hostPID determines if the policy allows the use of HostPID in the pod spec.
    /// +optional
    #[prost(bool, optional, tag="8")]
    pub host_pid: ::core::option::Option<bool>,
    /// hostIPC determines if the policy allows the use of HostIPC in the pod spec.
    /// +optional
    #[prost(bool, optional, tag="9")]
    pub host_ipc: ::core::option::Option<bool>,
    /// seLinux is the strategy that will dictate the allowable labels that may be set.
    #[prost(message, optional, tag="10")]
    pub se_linux: ::core::option::Option<SeLinuxStrategyOptions>,
    /// runAsUser is the strategy that will dictate the allowable RunAsUser values that may be set.
    #[prost(message, optional, tag="11")]
    pub run_as_user: ::core::option::Option<RunAsUserStrategyOptions>,
    /// RunAsGroup is the strategy that will dictate the allowable RunAsGroup values that may be set.
    /// If this field is omitted, the pod's RunAsGroup can take any value. This field requires the
    /// RunAsGroup feature gate to be enabled.
    /// +optional
    #[prost(message, optional, tag="22")]
    pub run_as_group: ::core::option::Option<RunAsGroupStrategyOptions>,
    /// supplementalGroups is the strategy that will dictate what supplemental groups are used by the SecurityContext.
    #[prost(message, optional, tag="12")]
    pub supplemental_groups: ::core::option::Option<SupplementalGroupsStrategyOptions>,
    /// fsGroup is the strategy that will dictate what fs group is used by the SecurityContext.
    #[prost(message, optional, tag="13")]
    pub fs_group: ::core::option::Option<FsGroupStrategyOptions>,
    /// readOnlyRootFilesystem when set to true will force containers to run with a read only root file
    /// system.  If the container specifically requests to run with a non-read only root file system
    /// the PSP should deny the pod.
    /// If set to false the container may run with a read only root file system if it wishes but it
    /// will not be forced to.
    /// +optional
    #[prost(bool, optional, tag="14")]
    pub read_only_root_filesystem: ::core::option::Option<bool>,
    /// defaultAllowPrivilegeEscalation controls the default setting for whether a
    /// process can gain more privileges than its parent process.
    /// +optional
    #[prost(bool, optional, tag="15")]
    pub default_allow_privilege_escalation: ::core::option::Option<bool>,
    /// allowPrivilegeEscalation determines if a pod can request to allow
    /// privilege escalation. If unspecified, defaults to true.
    /// +optional
    #[prost(bool, optional, tag="16")]
    pub allow_privilege_escalation: ::core::option::Option<bool>,
    /// allowedHostPaths is an allowlist of host paths. Empty indicates
    /// that all host paths may be used.
    /// +optional
    #[prost(message, repeated, tag="17")]
    pub allowed_host_paths: ::prost::alloc::vec::Vec<AllowedHostPath>,
    /// allowedFlexVolumes is an allowlist of Flexvolumes.  Empty or nil indicates that all
    /// Flexvolumes may be used.  This parameter is effective only when the usage of the Flexvolumes
    /// is allowed in the "volumes" field.
    /// +optional
    #[prost(message, repeated, tag="18")]
    pub allowed_flex_volumes: ::prost::alloc::vec::Vec<AllowedFlexVolume>,
    /// AllowedCSIDrivers is an allowlist of inline CSI drivers that must be explicitly set to be embedded within a pod spec.
    /// An empty value indicates that any CSI driver can be used for inline ephemeral volumes.
    /// This is a beta field, and is only honored if the API server enables the CSIInlineVolume feature gate.
    /// +optional
    #[prost(message, repeated, tag="23")]
    pub allowed_csi_drivers: ::prost::alloc::vec::Vec<AllowedCsiDriver>,
    /// allowedUnsafeSysctls is a list of explicitly allowed unsafe sysctls, defaults to none.
    /// Each entry is either a plain sysctl name or ends in "*" in which case it is considered
    /// as a prefix of allowed sysctls. Single * means all unsafe sysctls are allowed.
    /// Kubelet has to allowlist all allowed unsafe sysctls explicitly to avoid rejection.
    ///
    /// Examples:
    /// e.g. "foo/*" allows "foo/bar", "foo/baz", etc.
    /// e.g. "foo.*" allows "foo.bar", "foo.baz", etc.
    /// +optional
    #[prost(string, repeated, tag="19")]
    pub allowed_unsafe_sysctls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// forbiddenSysctls is a list of explicitly forbidden sysctls, defaults to none.
    /// Each entry is either a plain sysctl name or ends in "*" in which case it is considered
    /// as a prefix of forbidden sysctls. Single * means all sysctls are forbidden.
    ///
    /// Examples:
    /// e.g. "foo/*" forbids "foo/bar", "foo/baz", etc.
    /// e.g. "foo.*" forbids "foo.bar", "foo.baz", etc.
    /// +optional
    #[prost(string, repeated, tag="20")]
    pub forbidden_sysctls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// AllowedProcMountTypes is an allowlist of allowed ProcMountTypes.
    /// Empty or nil indicates that only the DefaultProcMountType may be used.
    /// This requires the ProcMountType feature flag to be enabled.
    /// +optional
    #[prost(string, repeated, tag="21")]
    pub allowed_proc_mount_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// runtimeClass is the strategy that will dictate the allowable RuntimeClasses for a pod.
    /// If this field is omitted, the pod's runtimeClassName field is unrestricted.
    /// Enforcement of this field depends on the RuntimeClass feature gate being enabled.
    /// +optional
    #[prost(message, optional, tag="24")]
    pub runtime_class: ::core::option::Option<RuntimeClassStrategyOptions>,
}
/// RunAsGroupStrategyOptions defines the strategy type and any options used to create the strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAsGroupStrategyOptions {
    /// rule is the strategy that will dictate the allowable RunAsGroup values that may be set.
    #[prost(string, optional, tag="1")]
    pub rule: ::core::option::Option<::prost::alloc::string::String>,
    /// ranges are the allowed ranges of gids that may be used. If you would like to force a single gid
    /// then supply a single range with the same start and end. Required for MustRunAs.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub ranges: ::prost::alloc::vec::Vec<IdRange>,
}
/// RunAsUserStrategyOptions defines the strategy type and any options used to create the strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunAsUserStrategyOptions {
    /// rule is the strategy that will dictate the allowable RunAsUser values that may be set.
    #[prost(string, optional, tag="1")]
    pub rule: ::core::option::Option<::prost::alloc::string::String>,
    /// ranges are the allowed ranges of uids that may be used. If you would like to force a single uid
    /// then supply a single range with the same start and end. Required for MustRunAs.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub ranges: ::prost::alloc::vec::Vec<IdRange>,
}
/// RuntimeClassStrategyOptions define the strategy that will dictate the allowable RuntimeClasses
/// for a pod.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeClassStrategyOptions {
    /// allowedRuntimeClassNames is an allowlist of RuntimeClass names that may be specified on a pod.
    /// A value of "*" means that any RuntimeClass name is allowed, and must be the only item in the
    /// list. An empty list requires the RuntimeClassName field to be unset.
    #[prost(string, repeated, tag="1")]
    pub allowed_runtime_class_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// defaultRuntimeClassName is the default RuntimeClassName to set on the pod.
    /// The default MUST be allowed by the allowedRuntimeClassNames list.
    /// A value of nil does not mutate the Pod.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub default_runtime_class_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// SELinuxStrategyOptions defines the strategy type and any options used to create the strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeLinuxStrategyOptions {
    /// rule is the strategy that will dictate the allowable labels that may be set.
    #[prost(string, optional, tag="1")]
    pub rule: ::core::option::Option<::prost::alloc::string::String>,
    /// seLinuxOptions required to run as; required for MustRunAs
    /// More info: https://kubernetes.io/docs/tasks/configure-pod-container/security-context/
    /// +optional
    #[prost(message, optional, tag="2")]
    pub se_linux_options: ::core::option::Option<super::super::core::v1::SeLinuxOptions>,
}
/// SupplementalGroupsStrategyOptions defines the strategy type and options used to create the strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplementalGroupsStrategyOptions {
    /// rule is the strategy that will dictate what supplemental groups is used in the SecurityContext.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub rule: ::core::option::Option<::prost::alloc::string::String>,
    /// ranges are the allowed ranges of supplemental groups.  If you would like to force a single
    /// supplemental group then supply a single range with the same start and end. Required for MustRunAs.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub ranges: ::prost::alloc::vec::Vec<IdRange>,
}

impl crate::Resource for PodDisruptionBudget {
    const API_VERSION: &'static str = "policy/v1beta1";
    const GROUP: &'static str = "policy";
    const VERSION: &'static str = "v1beta1";
    const KIND: &'static str = "PodDisruptionBudget";
    const NAME: &'static str = "poddisruptionbudgets";
}
impl crate::HasMetadata for PodDisruptionBudget {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for PodDisruptionBudget {
    type Spec = crate::api::policy::v1beta1::PodDisruptionBudgetSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for PodDisruptionBudget {
    type Status = crate::api::policy::v1beta1::PodDisruptionBudgetStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for PodDisruptionBudget {
    type Condition = crate::apimachinery::pkg::apis::meta::v1::Condition;
    fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
        self.status.as_ref().map(|s| s.conditions.as_slice())
    }
    fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
        self.status
            .as_mut()
            .and_then(|s| Some(s.conditions.as_mut()))
    }
}


impl crate::Resource for PodSecurityPolicy {
    const API_VERSION: &'static str = "policy/v1beta1";
    const GROUP: &'static str = "policy";
    const VERSION: &'static str = "v1beta1";
    const KIND: &'static str = "PodSecurityPolicy";
    const NAME: &'static str = "podsecuritypolicies";
}
impl crate::HasMetadata for PodSecurityPolicy {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for PodSecurityPolicy {
    type Spec = crate::api::policy::v1beta1::PodSecurityPolicySpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}

