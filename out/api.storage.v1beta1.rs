/// CSIDriver captures information about a Container Storage Interface (CSI)
/// volume driver deployed on the cluster.
/// CSI drivers do not need to create the CSIDriver object directly. Instead they may use the
/// cluster-driver-registrar sidecar container. When deployed with a CSI driver it automatically
/// creates a CSIDriver object representing the driver.
/// Kubernetes attach detach controller uses this object to determine whether attach is required.
/// Kubelet uses this object to determine whether pod information needs to be passed on mount.
/// CSIDriver objects are non-namespaced.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiDriver {
    /// Standard object metadata.
    /// metadata.Name indicates the name of the CSI driver that this object
    /// refers to; it MUST be the same name returned by the CSI GetPluginName()
    /// call for that driver.
    /// The driver name must be 63 characters or less, beginning and ending with
    /// an alphanumeric character ([a-z0-9A-Z]) with dashes (-), dots (.), and
    /// alphanumerics between.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the CSI Driver.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<CsiDriverSpec>,
}
/// CSIDriverList is a collection of CSIDriver objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiDriverList {
    /// Standard list metadata
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of CSIDriver
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CsiDriver>,
}
/// CSIDriverSpec is the specification of a CSIDriver.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiDriverSpec {
    /// attachRequired indicates this CSI volume driver requires an attach
    /// operation (because it implements the CSI ControllerPublishVolume()
    /// method), and that the Kubernetes attach detach controller should call
    /// the attach volume interface which checks the volumeattachment status
    /// and waits until the volume is attached before proceeding to mounting.
    /// The CSI external-attacher coordinates with CSI volume driver and updates
    /// the volumeattachment status when the attach operation is complete.
    /// If the CSIDriverRegistry feature gate is enabled and the value is
    /// specified to false, the attach operation will be skipped.
    /// Otherwise the attach operation will be called.
    ///
    /// This field is immutable.
    ///
    /// +optional
    #[prost(bool, optional, tag="1")]
    pub attach_required: ::core::option::Option<bool>,
    /// If set to true, podInfoOnMount indicates this CSI volume driver
    /// requires additional pod information (like podName, podUID, etc.) during
    /// mount operations.
    /// If set to false, pod information will not be passed on mount.
    /// Default is false.
    /// The CSI driver specifies podInfoOnMount as part of driver deployment.
    /// If true, Kubelet will pass pod information as VolumeContext in the CSI
    /// NodePublishVolume() calls.
    /// The CSI driver is responsible for parsing and validating the information
    /// passed in as VolumeContext.
    /// The following VolumeConext will be passed if podInfoOnMount is set to true.
    /// This list might grow, but the prefix will be used.
    /// "csi.storage.k8s.io/pod.name": pod.Name
    /// "csi.storage.k8s.io/pod.namespace": pod.Namespace
    /// "csi.storage.k8s.io/pod.uid": string(pod.UID)
    /// "csi.storage.k8s.io/ephemeral": "true" if the volume is an ephemeral inline volume
    ///                                 defined by a CSIVolumeSource, otherwise "false"
    ///
    /// "csi.storage.k8s.io/ephemeral" is a new feature in Kubernetes 1.16. It is only
    /// required for drivers which support both the "Persistent" and "Ephemeral" VolumeLifecycleMode.
    /// Other drivers can leave pod info disabled and/or ignore this field.
    /// As Kubernetes 1.15 doesn't support this field, drivers can only support one mode when
    /// deployed on such a cluster and the deployment determines which mode that is, for example
    /// via a command line parameter of the driver.
    ///
    /// This field is immutable.
    ///
    /// +optional
    #[prost(bool, optional, tag="2")]
    pub pod_info_on_mount: ::core::option::Option<bool>,
    /// VolumeLifecycleModes defines what kind of volumes this CSI volume driver supports.
    /// The default if the list is empty is "Persistent", which is the usage
    /// defined by the CSI specification and implemented in Kubernetes via the usual
    /// PV/PVC mechanism.
    /// The other mode is "Ephemeral". In this mode, volumes are defined inline
    /// inside the pod spec with CSIVolumeSource and their lifecycle is tied to
    /// the lifecycle of that pod. A driver has to be aware of this
    /// because it is only going to get a NodePublishVolume call for such a volume.
    /// For more information about implementing this mode, see
    /// https://kubernetes-csi.github.io/docs/ephemeral-local-volumes.html
    /// A driver can support one or more of these modes and
    /// more modes may be added in the future.
    ///
    /// This field is immutable.
    ///
    /// +optional
    #[prost(string, repeated, tag="3")]
    pub volume_lifecycle_modes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If set to true, storageCapacity indicates that the CSI
    /// volume driver wants pod scheduling to consider the storage
    /// capacity that the driver deployment will report by creating
    /// CSIStorageCapacity objects with capacity information.
    ///
    /// The check can be enabled immediately when deploying a driver.
    /// In that case, provisioning new volumes with late binding
    /// will pause until the driver deployment has published
    /// some suitable CSIStorageCapacity object.
    ///
    /// Alternatively, the driver can be deployed with the field
    /// unset or false and it can be flipped later when storage
    /// capacity information has been published.
    ///
    /// This field is immutable.
    ///
    /// This is a beta field and only available when the CSIStorageCapacity
    /// feature is enabled. The default is false.
    ///
    /// +optional
    /// +featureGate=CSIStorageCapacity
    #[prost(bool, optional, tag="4")]
    pub storage_capacity: ::core::option::Option<bool>,
    /// Defines if the underlying volume supports changing ownership and
    /// permission of the volume before being mounted.
    /// Refer to the specific FSGroupPolicy values for additional details.
    /// This field is beta, and is only honored by servers
    /// that enable the CSIVolumeFSGroupPolicy feature gate.
    ///
    /// This field is immutable.
    ///
    /// Defaults to ReadWriteOnceWithFSType, which will examine each volume
    /// to determine if Kubernetes should modify ownership and permissions of the volume.
    /// With the default policy the defined fsGroup will only be applied
    /// if a fstype is defined and the volume's access mode contains ReadWriteOnce.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub fs_group_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// TokenRequests indicates the CSI driver needs pods' service account
    /// tokens it is mounting volume for to do necessary authentication. Kubelet
    /// will pass the tokens in VolumeContext in the CSI NodePublishVolume calls.
    /// The CSI driver should parse and validate the following VolumeContext:
    /// "csi.storage.k8s.io/serviceAccount.tokens": {
    ///   "<audience>": {
    ///     "token": <token>,
    ///     "expirationTimestamp": <expiration timestamp in RFC3339>,
    ///   },
    ///   ...
    /// }
    ///
    /// Note: Audience in each TokenRequest should be different and at
    /// most one token is empty string. To receive a new token after expiry,
    /// RequiresRepublish can be used to trigger NodePublishVolume periodically.
    ///
    /// +optional
    /// +listType=atomic
    #[prost(message, repeated, tag="6")]
    pub token_requests: ::prost::alloc::vec::Vec<TokenRequest>,
    /// RequiresRepublish indicates the CSI driver wants `NodePublishVolume`
    /// being periodically called to reflect any possible change in the mounted
    /// volume. This field defaults to false.
    ///
    /// Note: After a successful initial NodePublishVolume call, subsequent calls
    /// to NodePublishVolume should only update the contents of the volume. New
    /// mount points will not be seen by a running container.
    ///
    /// +optional
    #[prost(bool, optional, tag="7")]
    pub requires_republish: ::core::option::Option<bool>,
}
/// DEPRECATED - This group version of CSINode is deprecated by storage/v1/CSINode.
/// See the release notes for more information.
/// CSINode holds information about all CSI drivers installed on a node.
/// CSI drivers do not need to create the CSINode object directly. As long as
/// they use the node-driver-registrar sidecar container, the kubelet will
/// automatically populate the CSINode object for the CSI driver as part of
/// kubelet plugin registration.
/// CSINode has the same name as a node. If the object is missing, it means either
/// there are no CSI Drivers available on the node, or the Kubelet version is low
/// enough that it doesn't create this object.
/// CSINode has an OwnerReference that points to the corresponding node object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiNode {
    /// metadata.name must be the Kubernetes node name.
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec is the specification of CSINode
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<CsiNodeSpec>,
}
/// CSINodeDriver holds information about the specification of one CSI driver installed on a node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiNodeDriver {
    /// This is the name of the CSI driver that this object refers to.
    /// This MUST be the same name returned by the CSI GetPluginName() call for
    /// that driver.
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// nodeID of the node from the driver point of view.
    /// This field enables Kubernetes to communicate with storage systems that do
    /// not share the same nomenclature for nodes. For example, Kubernetes may
    /// refer to a given node as "node1", but the storage system may refer to
    /// the same node as "nodeA". When Kubernetes issues a command to the storage
    /// system to attach a volume to a specific node, it can use this field to
    /// refer to the node name using the ID that the storage system will
    /// understand, e.g. "nodeA" instead of "node1". This field is required.
    #[prost(string, optional, tag="2")]
    pub node_id: ::core::option::Option<::prost::alloc::string::String>,
    /// topologyKeys is the list of keys supported by the driver.
    /// When a driver is initialized on a cluster, it provides a set of topology
    /// keys that it understands (e.g. "company.com/zone", "company.com/region").
    /// When a driver is initialized on a node, it provides the same topology keys
    /// along with values. Kubelet will expose these topology keys as labels
    /// on its own node object.
    /// When Kubernetes does topology aware provisioning, it can use this list to
    /// determine which labels it should retrieve from the node object and pass
    /// back to the driver.
    /// It is possible for different nodes to use different topology keys.
    /// This can be empty if driver does not support topology.
    /// +optional
    #[prost(string, repeated, tag="3")]
    pub topology_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// allocatable represents the volume resources of a node that are available for scheduling.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub allocatable: ::core::option::Option<VolumeNodeResources>,
}
/// CSINodeList is a collection of CSINode objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiNodeList {
    /// Standard list metadata
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of CSINode
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CsiNode>,
}
/// CSINodeSpec holds information about the specification of all CSI drivers installed on a node
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiNodeSpec {
    /// drivers is a list of information of all CSI Drivers existing on a node.
    /// If all drivers in the list are uninstalled, this can become empty.
    /// +patchMergeKey=name
    /// +patchStrategy=merge
    #[prost(message, repeated, tag="1")]
    pub drivers: ::prost::alloc::vec::Vec<CsiNodeDriver>,
}
/// CSIStorageCapacity stores the result of one CSI GetCapacity call.
/// For a given StorageClass, this describes the available capacity in a
/// particular topology segment.  This can be used when considering where to
/// instantiate new PersistentVolumes.
///
/// For example this can express things like:
/// - StorageClass "standard" has "1234 GiB" available in "topology.kubernetes.io/zone=us-east1"
/// - StorageClass "localssd" has "10 GiB" available in "kubernetes.io/hostname=knode-abc123"
///
/// The following three cases all imply that no capacity is available for
/// a certain combination:
/// - no object exists with suitable topology and storage class name
/// - such an object exists, but the capacity is unset
/// - such an object exists, but the capacity is zero
///
/// The producer of these objects can decide which approach is more suitable.
///
/// They are consumed by the kube-scheduler if the CSIStorageCapacity beta feature gate
/// is enabled there and a CSI driver opts into capacity-aware scheduling with
/// CSIDriver.StorageCapacity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiStorageCapacity {
    /// Standard object's metadata. The name has no particular meaning. It must be
    /// be a DNS subdomain (dots allowed, 253 characters). To ensure that
    /// there are no conflicts with other CSI drivers on the cluster, the recommendation
    /// is to use csisc-<uuid>, a generated name, or a reverse-domain name which ends
    /// with the unique CSI driver name.
    ///
    /// Objects are namespaced.
    ///
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// NodeTopology defines which nodes have access to the storage
    /// for which capacity was reported. If not set, the storage is
    /// not accessible from any node in the cluster. If empty, the
    /// storage is accessible from all nodes. This field is
    /// immutable.
    ///
    /// +optional
    #[prost(message, optional, tag="2")]
    pub node_topology: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::LabelSelector>,
    /// The name of the StorageClass that the reported capacity applies to.
    /// It must meet the same requirements as the name of a StorageClass
    /// object (non-empty, DNS subdomain). If that object no longer exists,
    /// the CSIStorageCapacity object is obsolete and should be removed by its
    /// creator.
    /// This field is immutable.
    #[prost(string, optional, tag="3")]
    pub storage_class_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Capacity is the value reported by the CSI driver in its GetCapacityResponse
    /// for a GetCapacityRequest with topology and parameters that match the
    /// previous fields.
    ///
    /// The semantic is currently (CSI spec 1.2) defined as:
    /// The available capacity, in bytes, of the storage that can be used
    /// to provision volumes. If not set, that information is currently
    /// unavailable and treated like zero capacity.
    ///
    /// +optional
    #[prost(message, optional, tag="4")]
    pub capacity: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
    /// MaximumVolumeSize is the value reported by the CSI driver in its GetCapacityResponse
    /// for a GetCapacityRequest with topology and parameters that match the
    /// previous fields.
    ///
    /// This is defined since CSI spec 1.4.0 as the largest size
    /// that may be used in a
    /// CreateVolumeRequest.capacity_range.required_bytes field to
    /// create a volume with the same parameters as those in
    /// GetCapacityRequest. The corresponding value in the Kubernetes
    /// API is ResourceRequirements.Requests in a volume claim.
    ///
    /// +optional
    #[prost(message, optional, tag="5")]
    pub maximum_volume_size: ::core::option::Option<super::super::super::apimachinery::pkg::api::resource::Quantity>,
}
/// CSIStorageCapacityList is a collection of CSIStorageCapacity objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsiStorageCapacityList {
    /// Standard list metadata
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of CSIStorageCapacity objects.
    /// +listType=map
    /// +listMapKey=name
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<CsiStorageCapacity>,
}
/// StorageClass describes the parameters for a class of storage for
/// which PersistentVolumes can be dynamically provisioned.
///
/// StorageClasses are non-namespaced; the name of the storage class
/// according to etcd is in ObjectMeta.Name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageClass {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Provisioner indicates the type of the provisioner.
    #[prost(string, optional, tag="2")]
    pub provisioner: ::core::option::Option<::prost::alloc::string::String>,
    /// Parameters holds the parameters for the provisioner that should
    /// create volumes of this storage class.
    /// +optional
    #[prost(map="string, string", tag="3")]
    pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Dynamically provisioned PersistentVolumes of this storage class are
    /// created with this reclaimPolicy. Defaults to Delete.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reclaim_policy: ::core::option::Option<::prost::alloc::string::String>,
    /// Dynamically provisioned PersistentVolumes of this storage class are
    /// created with these mountOptions, e.g. ["ro", "soft"]. Not validated -
    /// mount of the PVs will simply fail if one is invalid.
    /// +optional
    #[prost(string, repeated, tag="5")]
    pub mount_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// AllowVolumeExpansion shows whether the storage class allow volume expand
    /// +optional
    #[prost(bool, optional, tag="6")]
    pub allow_volume_expansion: ::core::option::Option<bool>,
    /// VolumeBindingMode indicates how PersistentVolumeClaims should be
    /// provisioned and bound.  When unset, VolumeBindingImmediate is used.
    /// This field is only honored by servers that enable the VolumeScheduling feature.
    /// +optional
    #[prost(string, optional, tag="7")]
    pub volume_binding_mode: ::core::option::Option<::prost::alloc::string::String>,
    /// Restrict the node topologies where volumes can be dynamically provisioned.
    /// Each volume plugin defines its own supported topology specifications.
    /// An empty TopologySelectorTerm list means there is no topology restriction.
    /// This field is only honored by servers that enable the VolumeScheduling feature.
    /// +optional
    /// +listType=atomic
    #[prost(message, repeated, tag="8")]
    pub allowed_topologies: ::prost::alloc::vec::Vec<super::super::core::v1::TopologySelectorTerm>,
}
/// StorageClassList is a collection of storage classes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageClassList {
    /// Standard list metadata
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of StorageClasses
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<StorageClass>,
}
/// TokenRequest contains parameters of a service account token.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenRequest {
    /// Audience is the intended audience of the token in "TokenRequestSpec".
    /// It will default to the audiences of kube apiserver.
    #[prost(string, optional, tag="1")]
    pub audience: ::core::option::Option<::prost::alloc::string::String>,
    /// ExpirationSeconds is the duration of validity of the token in "TokenRequestSpec".
    /// It has the same default value of "ExpirationSeconds" in "TokenRequestSpec"
    ///
    /// +optional
    #[prost(int64, optional, tag="2")]
    pub expiration_seconds: ::core::option::Option<i64>,
}
/// VolumeAttachment captures the intent to attach or detach the specified volume
/// to/from the specified node.
///
/// VolumeAttachment objects are non-namespaced.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeAttachment {
    /// Standard object metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the desired attach/detach volume behavior.
    /// Populated by the Kubernetes system.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<VolumeAttachmentSpec>,
    /// Status of the VolumeAttachment request.
    /// Populated by the entity completing the attach or detach
    /// operation, i.e. the external-attacher.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<VolumeAttachmentStatus>,
}
/// VolumeAttachmentList is a collection of VolumeAttachment objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeAttachmentList {
    /// Standard list metadata
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of VolumeAttachments
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<VolumeAttachment>,
}
/// VolumeAttachmentSource represents a volume that should be attached.
/// Right now only PersistenVolumes can be attached via external attacher,
/// in future we may allow also inline volumes in pods.
/// Exactly one member can be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeAttachmentSource {
    /// Name of the persistent volume to attach.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub persistent_volume_name: ::core::option::Option<::prost::alloc::string::String>,
    /// inlineVolumeSpec contains all the information necessary to attach
    /// a persistent volume defined by a pod's inline VolumeSource. This field
    /// is populated only for the CSIMigration feature. It contains
    /// translated fields from a pod's inline VolumeSource to a
    /// PersistentVolumeSpec. This field is beta-level and is only
    /// honored by servers that enabled the CSIMigration feature.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub inline_volume_spec: ::core::option::Option<super::super::core::v1::PersistentVolumeSpec>,
}
/// VolumeAttachmentSpec is the specification of a VolumeAttachment request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeAttachmentSpec {
    /// Attacher indicates the name of the volume driver that MUST handle this
    /// request. This is the name returned by GetPluginName().
    #[prost(string, optional, tag="1")]
    pub attacher: ::core::option::Option<::prost::alloc::string::String>,
    /// Source represents the volume that should be attached.
    #[prost(message, optional, tag="2")]
    pub source: ::core::option::Option<VolumeAttachmentSource>,
    /// The node that the volume should be attached to.
    #[prost(string, optional, tag="3")]
    pub node_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// VolumeAttachmentStatus is the status of a VolumeAttachment request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeAttachmentStatus {
    /// Indicates the volume is successfully attached.
    /// This field must only be set by the entity completing the attach
    /// operation, i.e. the external-attacher.
    #[prost(bool, optional, tag="1")]
    pub attached: ::core::option::Option<bool>,
    /// Upon successful attach, this field is populated with any
    /// information returned by the attach operation that must be passed
    /// into subsequent WaitForAttach or Mount calls.
    /// This field must only be set by the entity completing the attach
    /// operation, i.e. the external-attacher.
    /// +optional
    #[prost(map="string, string", tag="2")]
    pub attachment_metadata: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The last error encountered during attach operation, if any.
    /// This field must only be set by the entity completing the attach
    /// operation, i.e. the external-attacher.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub attach_error: ::core::option::Option<VolumeError>,
    /// The last error encountered during detach operation, if any.
    /// This field must only be set by the entity completing the detach
    /// operation, i.e. the external-attacher.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub detach_error: ::core::option::Option<VolumeError>,
}
/// VolumeError captures an error encountered during a volume operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeError {
    /// Time the error was encountered.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// String detailing the error encountered during Attach or Detach operation.
    /// This string may be logged, so it should not contain sensitive
    /// information.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// VolumeNodeResources is a set of resource limits for scheduling of volumes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeNodeResources {
    /// Maximum number of unique volumes managed by the CSI driver that can be used on a node.
    /// A volume that is both attached and mounted on a node is considered to be used once, not twice.
    /// The same rule applies for a unique volume that is shared among multiple pods on the same node.
    /// If this field is nil, then the supported number of volumes on this node is unbounded.
    /// +optional
    #[prost(int32, optional, tag="1")]
    pub count: ::core::option::Option<i32>,
}
