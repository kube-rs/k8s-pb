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
    /// PersistentVolumeSpec. This field is alpha-level and is only
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
    /// This string maybe logged, so it should not contain sensitive
    /// information.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}

impl crate::Resource for CsiStorageCapacity {
    const API_VERSION: &'static str = "storage.k8s.io/v1alpha1";
    const GROUP: &'static str = "storage.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "CSIStorageCapacity";
    const NAME: &'static str = "csistoragecapacities";
}
impl crate::HasMetadata for CsiStorageCapacity {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}


impl crate::Resource for VolumeAttachment {
    const API_VERSION: &'static str = "storage.k8s.io/v1alpha1";
    const GROUP: &'static str = "storage.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "VolumeAttachment";
    const NAME: &'static str = "volumeattachments";
}
impl crate::HasMetadata for VolumeAttachment {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for VolumeAttachment {
    type Spec = crate::api::storage::v1alpha1::VolumeAttachmentSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for VolumeAttachment {
    type Status = crate::api::storage::v1alpha1::VolumeAttachmentStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}

