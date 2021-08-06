/// DEPRECATED - This group version of ControllerRevision is deprecated by apps/v1beta2/ControllerRevision. See the
/// release notes for more information.
/// ControllerRevision implements an immutable snapshot of state data. Clients
/// are responsible for serializing and deserializing the objects that contain
/// their internal state.
/// Once a ControllerRevision has been successfully created, it can not be updated.
/// The API Server will fail validation of all requests that attempt to mutate
/// the Data field. ControllerRevisions may, however, be deleted. Note that, due to its use by both
/// the DaemonSet and StatefulSet controllers for update and rollback, this object is beta. However,
/// it may be subject to name and representation changes in future releases, and clients should not
/// depend on its stability. It is primarily for internal use by controllers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControllerRevision {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Data is the serialized representation of the state.
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<super::super::super::apimachinery::pkg::runtime::RawExtension>,
    /// Revision indicates the revision of the state represented by Data.
    #[prost(int64, optional, tag="3")]
    pub revision: ::core::option::Option<i64>,
}
/// ControllerRevisionList is a resource containing a list of ControllerRevision objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControllerRevisionList {
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of ControllerRevisions
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ControllerRevision>,
}
/// DEPRECATED - This group version of Deployment is deprecated by apps/v1beta2/Deployment. See the release notes for
/// more information.
/// Deployment enables declarative updates for Pods and ReplicaSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Standard object metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired behavior of the Deployment.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<DeploymentSpec>,
    /// Most recently observed status of the Deployment.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<DeploymentStatus>,
}
/// DeploymentCondition describes the state of a deployment at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentCondition {
    /// Type of deployment condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// The last time this condition was updated.
    #[prost(message, optional, tag="6")]
    pub last_update_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// Last time the condition transitioned from one status to another.
    #[prost(message, optional, tag="7")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// DeploymentList is a list of Deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentList {
    /// Standard list metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of Deployments.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Deployment>,
}
/// DEPRECATED.
/// DeploymentRollback stores the information required to rollback a deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentRollback {
    /// Required: This must match the Name of a deployment.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The annotations to be updated to a deployment
    /// +optional
    #[prost(map="string, string", tag="2")]
    pub updated_annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The config of this deployment rollback.
    #[prost(message, optional, tag="3")]
    pub rollback_to: ::core::option::Option<RollbackConfig>,
}
/// DeploymentSpec is the specification of the desired behavior of the Deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentSpec {
    /// Number of desired pods. This is a pointer to distinguish between explicit
    /// zero and not specified. Defaults to 1.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// Label selector for pods. Existing ReplicaSets whose pods are
    /// selected by this will be the ones affected by this deployment.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// Template describes the pods that will be created.
    #[prost(message, optional, tag="3")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// The deployment strategy to use to replace existing pods with new ones.
    /// +optional
    /// +patchStrategy=retainKeys
    #[prost(message, optional, tag="4")]
    pub strategy: ::core::option::Option<DeploymentStrategy>,
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing, for it to be considered available.
    /// Defaults to 0 (pod will be considered available as soon as it is ready)
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub min_ready_seconds: ::core::option::Option<i32>,
    /// The number of old ReplicaSets to retain to allow rollback.
    /// This is a pointer to distinguish between explicit zero and not specified.
    /// Defaults to 2.
    /// +optional
    #[prost(int32, optional, tag="6")]
    pub revision_history_limit: ::core::option::Option<i32>,
    /// Indicates that the deployment is paused.
    /// +optional
    #[prost(bool, optional, tag="7")]
    pub paused: ::core::option::Option<bool>,
    /// DEPRECATED.
    /// The config this deployment is rolling back to. Will be cleared after rollback is done.
    /// +optional
    #[prost(message, optional, tag="8")]
    pub rollback_to: ::core::option::Option<RollbackConfig>,
    /// The maximum time in seconds for a deployment to make progress before it
    /// is considered to be failed. The deployment controller will continue to
    /// process failed deployments and a condition with a ProgressDeadlineExceeded
    /// reason will be surfaced in the deployment status. Note that progress will
    /// not be estimated during the time a deployment is paused. Defaults to 600s.
    /// +optional
    #[prost(int32, optional, tag="9")]
    pub progress_deadline_seconds: ::core::option::Option<i32>,
}
/// DeploymentStatus is the most recently observed status of the Deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentStatus {
    /// The generation observed by the deployment controller.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// Total number of non-terminated pods targeted by this deployment (their labels match the selector).
    /// +optional
    #[prost(int32, optional, tag="2")]
    pub replicas: ::core::option::Option<i32>,
    /// Total number of non-terminated pods targeted by this deployment that have the desired template spec.
    /// +optional
    #[prost(int32, optional, tag="3")]
    pub updated_replicas: ::core::option::Option<i32>,
    /// Total number of ready pods targeted by this deployment.
    /// +optional
    #[prost(int32, optional, tag="7")]
    pub ready_replicas: ::core::option::Option<i32>,
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this deployment.
    /// +optional
    #[prost(int32, optional, tag="4")]
    pub available_replicas: ::core::option::Option<i32>,
    /// Total number of unavailable pods targeted by this deployment. This is the total number of
    /// pods that are still required for the deployment to have 100% available capacity. They may
    /// either be pods that are running but not yet available or pods that still have not been created.
    /// +optional
    #[prost(int32, optional, tag="5")]
    pub unavailable_replicas: ::core::option::Option<i32>,
    /// Represents the latest available observations of a deployment's current state.
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="6")]
    pub conditions: ::prost::alloc::vec::Vec<DeploymentCondition>,
    /// Count of hash collisions for the Deployment. The Deployment controller uses this
    /// field as a collision avoidance mechanism when it needs to create the name for the
    /// newest ReplicaSet.
    /// +optional
    #[prost(int32, optional, tag="8")]
    pub collision_count: ::core::option::Option<i32>,
}
/// DeploymentStrategy describes how to replace existing pods with new ones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeploymentStrategy {
    /// Type of deployment. Can be "Recreate" or "RollingUpdate". Default is RollingUpdate.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Rolling update config params. Present only if DeploymentStrategyType =
    /// RollingUpdate.
    /// ---
    /// TODO: Update this to follow our convention for oneOf, whatever we decide it
    /// to be.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub rolling_update: ::core::option::Option<RollingUpdateDeployment>,
}
/// DEPRECATED.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackConfig {
    /// The revision to rollback to. If set to 0, rollback to the last revision.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub revision: ::core::option::Option<i64>,
}
/// Spec to control the desired behavior of rolling update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollingUpdateDeployment {
    /// The maximum number of pods that can be unavailable during the update.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// Absolute number is calculated from percentage by rounding down.
    /// This can not be 0 if MaxSurge is 0.
    /// Defaults to 25%.
    /// Example: when this is set to 30%, the old ReplicaSet can be scaled down to 70% of desired pods
    /// immediately when the rolling update starts. Once new pods are ready, old ReplicaSet
    /// can be scaled down further, followed by scaling up the new ReplicaSet, ensuring
    /// that the total number of pods available at all times during the update is at
    /// least 70% of desired pods.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub max_unavailable: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// The maximum number of pods that can be scheduled above the desired number of
    /// pods.
    /// Value can be an absolute number (ex: 5) or a percentage of desired pods (ex: 10%).
    /// This can not be 0 if MaxUnavailable is 0.
    /// Absolute number is calculated from percentage by rounding up.
    /// Defaults to 25%.
    /// Example: when this is set to 30%, the new ReplicaSet can be scaled up immediately when
    /// the rolling update starts, such that the total number of old and new pods do not exceed
    /// 130% of desired pods. Once old pods have been killed,
    /// new ReplicaSet can be scaled up further, ensuring that total number of pods running
    /// at any time during the update is at most 130% of desired pods.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub max_surge: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
}
/// RollingUpdateStatefulSetStrategy is used to communicate parameter for RollingUpdateStatefulSetStrategyType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollingUpdateStatefulSetStrategy {
    /// Partition indicates the ordinal at which the StatefulSet should be
    /// partitioned.
    #[prost(int32, optional, tag="1")]
    pub partition: ::core::option::Option<i32>,
}
/// Scale represents a scaling request for a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scale {
    /// Standard object metadata; More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// defines the behavior of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ScaleSpec>,
    /// current status of the scale. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status. Read-only.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<ScaleStatus>,
}
/// ScaleSpec describes the attributes of a scale subresource
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleSpec {
    /// desired number of instances for the scaled object.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
}
/// ScaleStatus represents the current status of a scale subresource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScaleStatus {
    /// actual number of observed instances of the scaled object.
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// label query over pods that should match the replicas count. More info: http://kubernetes.io/docs/user-guide/labels#label-selectors
    /// +optional
    #[prost(map="string, string", tag="2")]
    pub selector: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// label selector for pods that should match the replicas count. This is a serializated
    /// version of both map-based and more expressive set-based selectors. This is done to
    /// avoid introspection in the clients. The string will be in the same format as the
    /// query-param syntax. If the target type only supports map-based selectors, both this
    /// field and map-based selector field are populated.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    /// +optional
    #[prost(string, optional, tag="3")]
    pub target_selector: ::core::option::Option<::prost::alloc::string::String>,
}
/// DEPRECATED - This group version of StatefulSet is deprecated by apps/v1beta2/StatefulSet. See the release notes for
/// more information.
/// StatefulSet represents a set of pods with consistent identities.
/// Identities are defined as:
///  - Network: A single stable DNS and hostname.
///  - Storage: As many VolumeClaims as requested.
/// The StatefulSet guarantees that a given network identity will always
/// map to the same storage identity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSet {
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec defines the desired identities of pods in this set.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<StatefulSetSpec>,
    /// Status is the current status of Pods in this StatefulSet. This data
    /// may be out of date by some window of time.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<StatefulSetStatus>,
}
/// StatefulSetCondition describes the state of a statefulset at a certain point.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetCondition {
    /// Type of statefulset condition.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag="2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// Last time the condition transitioned from one status to another.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub last_transition_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// StatefulSetList is a collection of StatefulSets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetList {
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<StatefulSet>,
}
/// A StatefulSetSpec is the specification of a StatefulSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetSpec {
    /// replicas is the desired number of replicas of the given Template.
    /// These are replicas in the sense that they are instantiations of the
    /// same Template, but individual replicas also have a consistent identity.
    /// If unspecified, defaults to 1.
    /// TODO: Consider a rename of this field.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub replicas: ::core::option::Option<i32>,
    /// selector is a label query over pods that should match the replica count.
    /// If empty, defaulted to labels on the pod template.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors
    /// +optional
    #[prost(message, optional, tag="2")]
    pub selector: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// template is the object that describes the pod that will be created if
    /// insufficient replicas are detected. Each pod stamped out by the StatefulSet
    /// will fulfill this Template, but have a unique identity from the rest
    /// of the StatefulSet.
    #[prost(message, optional, tag="3")]
    pub template: ::core::option::Option<super::super::core::v1::PodTemplateSpec>,
    /// volumeClaimTemplates is a list of claims that pods are allowed to reference.
    /// The StatefulSet controller is responsible for mapping network identities to
    /// claims in a way that maintains the identity of a pod. Every claim in
    /// this list must have at least one matching (by name) volumeMount in one
    /// container in the template. A claim in this list takes precedence over
    /// any volumes in the template, with the same name.
    /// TODO: Define the behavior if a claim already exists with the same name.
    /// +optional
    #[prost(message, repeated, tag="4")]
    pub volume_claim_templates: ::prost::alloc::vec::Vec<super::super::core::v1::PersistentVolumeClaim>,
    /// serviceName is the name of the service that governs this StatefulSet.
    /// This service must exist before the StatefulSet, and is responsible for
    /// the network identity of the set. Pods get DNS/hostnames that follow the
    /// pattern: pod-specific-string.serviceName.default.svc.cluster.local
    /// where "pod-specific-string" is managed by the StatefulSet controller.
    #[prost(string, optional, tag="5")]
    pub service_name: ::core::option::Option<::prost::alloc::string::String>,
    /// podManagementPolicy controls how pods are created during initial scale up,
    /// when replacing pods on nodes, or when scaling down. The default policy is
    /// `OrderedReady`, where pods are created in increasing order (pod-0, then
    /// pod-1, etc) and the controller will wait until each pod is ready before
    /// continuing. When scaling down, the pods are removed in the opposite order.
    /// The alternative policy is `Parallel` which will create pods in parallel
    /// to match the desired scale without waiting, and on scale down will delete
    /// all pods at once.
    /// +optional
    #[prost(string, optional, tag="6")]
    pub pod_management_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// updateStrategy indicates the StatefulSetUpdateStrategy that will be
    /// employed to update Pods in the StatefulSet when a revision is made to
    /// Template.
    #[prost(message, optional, tag="7")]
    pub update_strategy: ::core::option::Option<StatefulSetUpdateStrategy>,
    /// revisionHistoryLimit is the maximum number of revisions that will
    /// be maintained in the StatefulSet's revision history. The revision history
    /// consists of all revisions not represented by a currently applied
    /// StatefulSetSpec version. The default value is 10.
    #[prost(int32, optional, tag="8")]
    pub revision_history_limit: ::core::option::Option<i32>,
    /// Minimum number of seconds for which a newly created pod should be ready
    /// without any of its container crashing for it to be considered available.
    /// Defaults to 0 (pod will be considered available as soon as it is ready)
    /// This is an alpha field and requires enabling StatefulSetMinReadySeconds feature gate.
    /// +optional
    #[prost(int32, optional, tag="9")]
    pub min_ready_seconds: ::core::option::Option<i32>,
}
/// StatefulSetStatus represents the current state of a StatefulSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetStatus {
    /// observedGeneration is the most recent generation observed for this StatefulSet. It corresponds to the
    /// StatefulSet's generation, which is updated on mutation by the API Server.
    /// +optional
    #[prost(int64, optional, tag="1")]
    pub observed_generation: ::core::option::Option<i64>,
    /// replicas is the number of Pods created by the StatefulSet controller.
    #[prost(int32, optional, tag="2")]
    pub replicas: ::core::option::Option<i32>,
    /// readyReplicas is the number of Pods created by the StatefulSet controller that have a Ready Condition.
    #[prost(int32, optional, tag="3")]
    pub ready_replicas: ::core::option::Option<i32>,
    /// currentReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version
    /// indicated by currentRevision.
    #[prost(int32, optional, tag="4")]
    pub current_replicas: ::core::option::Option<i32>,
    /// updatedReplicas is the number of Pods created by the StatefulSet controller from the StatefulSet version
    /// indicated by updateRevision.
    #[prost(int32, optional, tag="5")]
    pub updated_replicas: ::core::option::Option<i32>,
    /// currentRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the
    /// sequence [0,currentReplicas).
    #[prost(string, optional, tag="6")]
    pub current_revision: ::core::option::Option<::prost::alloc::string::String>,
    /// updateRevision, if not empty, indicates the version of the StatefulSet used to generate Pods in the sequence
    /// [replicas-updatedReplicas,replicas)
    #[prost(string, optional, tag="7")]
    pub update_revision: ::core::option::Option<::prost::alloc::string::String>,
    /// collisionCount is the count of hash collisions for the StatefulSet. The StatefulSet controller
    /// uses this field as a collision avoidance mechanism when it needs to create the name for the
    /// newest ControllerRevision.
    /// +optional
    #[prost(int32, optional, tag="9")]
    pub collision_count: ::core::option::Option<i32>,
    /// Represents the latest available observations of a statefulset's current state.
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="10")]
    pub conditions: ::prost::alloc::vec::Vec<StatefulSetCondition>,
    /// Total number of available pods (ready for at least minReadySeconds) targeted by this StatefulSet.
    /// This is an alpha field and requires enabling StatefulSetMinReadySeconds feature gate.
    /// Remove omitempty when graduating to beta
    /// +optional
    #[prost(int32, optional, tag="11")]
    pub available_replicas: ::core::option::Option<i32>,
}
/// StatefulSetUpdateStrategy indicates the strategy that the StatefulSet
/// controller will use to perform updates. It includes any additional parameters
/// necessary to perform the update for the indicated strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatefulSetUpdateStrategy {
    /// Type indicates the type of the StatefulSetUpdateStrategy.
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// RollingUpdate is used to communicate parameters when Type is RollingUpdateStatefulSetStrategyType.
    #[prost(message, optional, tag="2")]
    pub rolling_update: ::core::option::Option<RollingUpdateStatefulSetStrategy>,
}
