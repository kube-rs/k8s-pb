/// AllocationResult contains attributes of an allocated resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationResult {
    /// ResourceHandles contain the state associated with an allocation that
    /// should be maintained throughout the lifetime of a claim. Each
    /// ResourceHandle contains data that should be passed to a specific kubelet
    /// plugin once it lands on a node. This data is returned by the driver
    /// after a successful allocation and is opaque to Kubernetes. Driver
    /// documentation may explain to users how to interpret this data if needed.
    ///
    /// Setting this field is optional. It has a maximum size of 32 entries.
    /// If null (or empty), it is assumed this allocation will be processed by a
    /// single kubelet plugin with no ResourceHandle data attached. The name of
    /// the kubelet plugin invoked will match the DriverName set in the
    /// ResourceClaimStatus this AllocationResult is embedded in.
    ///
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub resource_handles: ::prost::alloc::vec::Vec<ResourceHandle>,
    /// This field will get set by the resource driver after it has allocated
    /// the resource to inform the scheduler where it can schedule Pods using
    /// the ResourceClaim.
    ///
    /// Setting this field is optional. If null, the resource is available
    /// everywhere.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub available_on_nodes: ::core::option::Option<super::super::core::v1::NodeSelector>,
    /// Shareable determines whether the resource supports more
    /// than one consumer at a time.
    /// +optional
    #[prost(bool, optional, tag="3")]
    pub shareable: ::core::option::Option<bool>,
}
/// PodSchedulingContext objects hold information that is needed to schedule
/// a Pod with ResourceClaims that use "WaitForFirstConsumer" allocation
/// mode.
///
/// This is an alpha type and requires enabling the DynamicResourceAllocation
/// feature gate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSchedulingContext {
    /// Standard object metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec describes where resources for the Pod are needed.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<PodSchedulingContextSpec>,
    /// Status describes where resources for the Pod can be allocated.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<PodSchedulingContextStatus>,
}
/// PodSchedulingContextList is a collection of Pod scheduling objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSchedulingContextList {
    /// Standard list metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of PodSchedulingContext objects.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<PodSchedulingContext>,
}
/// PodSchedulingContextSpec describes where resources for the Pod are needed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSchedulingContextSpec {
    /// SelectedNode is the node for which allocation of ResourceClaims that
    /// are referenced by the Pod and that use "WaitForFirstConsumer"
    /// allocation is to be attempted.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub selected_node: ::core::option::Option<::prost::alloc::string::String>,
    /// PotentialNodes lists nodes where the Pod might be able to run.
    ///
    /// The size of this field is limited to 128. This is large enough for
    /// many clusters. Larger clusters may need more attempts to find a node
    /// that suits all pending resources. This may get increased in the
    /// future, but not reduced.
    ///
    /// +listType=set
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub potential_nodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PodSchedulingContextStatus describes where resources for the Pod can be allocated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSchedulingContextStatus {
    /// ResourceClaims describes resource availability for each
    /// pod.spec.resourceClaim entry where the corresponding ResourceClaim
    /// uses "WaitForFirstConsumer" allocation mode.
    ///
    /// +listType=map
    /// +listMapKey=name
    /// +optional
    #[prost(message, repeated, tag="1")]
    pub resource_claims: ::prost::alloc::vec::Vec<ResourceClaimSchedulingStatus>,
}
/// ResourceClaim describes which resources are needed by a resource consumer.
/// Its status tracks whether the resource has been allocated and what the
/// resulting attributes are.
///
/// This is an alpha type and requires enabling the DynamicResourceAllocation
/// feature gate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaim {
    /// Standard object metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec describes the desired attributes of a resource that then needs
    /// to be allocated. It can only be set once when creating the
    /// ResourceClaim.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ResourceClaimSpec>,
    /// Status describes whether the resource is available and with which
    /// attributes.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<ResourceClaimStatus>,
}
/// ResourceClaimConsumerReference contains enough information to let you
/// locate the consumer of a ResourceClaim. The user must be a resource in the same
/// namespace as the ResourceClaim.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimConsumerReference {
    /// APIGroup is the group for the resource being referenced. It is
    /// empty for the core API. This matches the group in the APIVersion
    /// that is used when creating the resources.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub api_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource is the type of resource being referenced, for example "pods".
    #[prost(string, optional, tag="3")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of resource being referenced.
    #[prost(string, optional, tag="4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// UID identifies exactly one incarnation of the resource.
    #[prost(string, optional, tag="5")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
}
/// ResourceClaimList is a collection of claims.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimList {
    /// Standard list metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of resource claims.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ResourceClaim>,
}
/// ResourceClaimParametersReference contains enough information to let you
/// locate the parameters for a ResourceClaim. The object must be in the same
/// namespace as the ResourceClaim.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimParametersReference {
    /// APIGroup is the group for the resource being referenced. It is
    /// empty for the core API. This matches the group in the APIVersion
    /// that is used when creating the resources.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub api_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Kind is the type of resource being referenced. This is the same
    /// value as in the parameter object's metadata, for example "ConfigMap".
    #[prost(string, optional, tag="2")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of resource being referenced.
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// ResourceClaimSchedulingStatus contains information about one particular
/// ResourceClaim with "WaitForFirstConsumer" allocation mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimSchedulingStatus {
    /// Name matches the pod.spec.resourceClaims[*].Name field.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// UnsuitableNodes lists nodes that the ResourceClaim cannot be
    /// allocated for.
    ///
    /// The size of this field is limited to 128, the same as for
    /// PodSchedulingSpec.PotentialNodes. This may get increased in the
    /// future, but not reduced.
    ///
    /// +listType=set
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub unsuitable_nodes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ResourceClaimSpec defines how a resource is to be allocated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimSpec {
    /// ResourceClassName references the driver and additional parameters
    /// via the name of a ResourceClass that was created as part of the
    /// driver deployment.
    #[prost(string, optional, tag="1")]
    pub resource_class_name: ::core::option::Option<::prost::alloc::string::String>,
    /// ParametersRef references a separate object with arbitrary parameters
    /// that will be used by the driver when allocating a resource for the
    /// claim.
    ///
    /// The object must be in the same namespace as the ResourceClaim.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub parameters_ref: ::core::option::Option<ResourceClaimParametersReference>,
    /// Allocation can start immediately or when a Pod wants to use the
    /// resource. "WaitForFirstConsumer" is the default.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub allocation_mode: ::core::option::Option<::prost::alloc::string::String>,
}
/// ResourceClaimStatus tracks whether the resource has been allocated and what
/// the resulting attributes are.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimStatus {
    /// DriverName is a copy of the driver name from the ResourceClass at
    /// the time when allocation started.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub driver_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Allocation is set by the resource driver once a resource or set of
    /// resources has been allocated successfully. If this is not specified, the
    /// resources have not been allocated yet.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub allocation: ::core::option::Option<AllocationResult>,
    /// ReservedFor indicates which entities are currently allowed to use
    /// the claim. A Pod which references a ResourceClaim which is not
    /// reserved for that Pod will not be started.
    ///
    /// There can be at most 32 such reservations. This may get increased in
    /// the future, but not reduced.
    ///
    /// +listType=map
    /// +listMapKey=uid
    /// +optional
    #[prost(message, repeated, tag="3")]
    pub reserved_for: ::prost::alloc::vec::Vec<ResourceClaimConsumerReference>,
    /// DeallocationRequested indicates that a ResourceClaim is to be
    /// deallocated.
    ///
    /// The driver then must deallocate this claim and reset the field
    /// together with clearing the Allocation field.
    ///
    /// While DeallocationRequested is set, no new consumers may be added to
    /// ReservedFor.
    /// +optional
    #[prost(bool, optional, tag="4")]
    pub deallocation_requested: ::core::option::Option<bool>,
}
/// ResourceClaimTemplate is used to produce ResourceClaim objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimTemplate {
    /// Standard object metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Describes the ResourceClaim that is to be generated.
    ///
    /// This field is immutable. A ResourceClaim will get created by the
    /// control plane for a Pod when needed and then not get updated
    /// anymore.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ResourceClaimTemplateSpec>,
}
/// ResourceClaimTemplateList is a collection of claim templates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimTemplateList {
    /// Standard list metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of resource claim templates.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ResourceClaimTemplate>,
}
/// ResourceClaimTemplateSpec contains the metadata and fields for a ResourceClaim.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClaimTemplateSpec {
    /// ObjectMeta may contain labels and annotations that will be copied into the PVC
    /// when creating it. No other fields are allowed and will be rejected during
    /// validation.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec for the ResourceClaim. The entire content is copied unchanged
    /// into the ResourceClaim that gets created from this template. The
    /// same fields as in a ResourceClaim are also valid here.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ResourceClaimSpec>,
}
/// ResourceClass is used by administrators to influence how resources
/// are allocated.
///
/// This is an alpha type and requires enabling the DynamicResourceAllocation
/// feature gate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClass {
    /// Standard object metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// DriverName defines the name of the dynamic resource driver that is
    /// used for allocation of a ResourceClaim that uses this class.
    ///
    /// Resource drivers have a unique name in forward domain order
    /// (acme.example.com).
    #[prost(string, optional, tag="2")]
    pub driver_name: ::core::option::Option<::prost::alloc::string::String>,
    /// ParametersRef references an arbitrary separate object that may hold
    /// parameters that will be used by the driver when allocating a
    /// resource that uses this class. A dynamic resource driver can
    /// distinguish between parameters stored here and and those stored in
    /// ResourceClaimSpec.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub parameters_ref: ::core::option::Option<ResourceClassParametersReference>,
    /// Only nodes matching the selector will be considered by the scheduler
    /// when trying to find a Node that fits a Pod when that Pod uses
    /// a ResourceClaim that has not been allocated yet.
    ///
    /// Setting this field is optional. If null, all nodes are candidates.
    /// +optional
    #[prost(message, optional, tag="4")]
    pub suitable_nodes: ::core::option::Option<super::super::core::v1::NodeSelector>,
}
/// ResourceClassList is a collection of classes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClassList {
    /// Standard list metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of resource classes.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ResourceClass>,
}
/// ResourceClassParametersReference contains enough information to let you
/// locate the parameters for a ResourceClass.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceClassParametersReference {
    /// APIGroup is the group for the resource being referenced. It is
    /// empty for the core API. This matches the group in the APIVersion
    /// that is used when creating the resources.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub api_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Kind is the type of resource being referenced. This is the same
    /// value as in the parameter object's metadata.
    #[prost(string, optional, tag="2")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of resource being referenced.
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace that contains the referenced resource. Must be empty
    /// for cluster-scoped resources and non-empty for namespaced
    /// resources.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
}
/// ResourceHandle holds opaque resource data for processing by a specific kubelet plugin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceHandle {
    /// DriverName specifies the name of the resource driver whose kubelet
    /// plugin should be invoked to process this ResourceHandle's data once it
    /// lands on a node. This may differ from the DriverName set in
    /// ResourceClaimStatus this ResourceHandle is embedded in.
    #[prost(string, optional, tag="1")]
    pub driver_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Data contains the opaque data associated with this ResourceHandle. It is
    /// set by the controller component of the resource driver whose name
    /// matches the DriverName set in the ResourceClaimStatus this
    /// ResourceHandle is embedded in. It is set at allocation time and is
    /// intended for processing by the kubelet plugin whose name matches
    /// the DriverName set in this ResourceHandle.
    ///
    /// The maximum size of this field is 16KiB. This may get increased in the
    /// future, but not reduced.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub data: ::core::option::Option<::prost::alloc::string::String>,
}

impl crate::Resource for PodSchedulingContext {
    const API_VERSION: &'static str = "resource.k8s.io/v1alpha2";
    const GROUP: &'static str = "resource.k8s.io";
    const VERSION: &'static str = "v1alpha2";
    const KIND: &'static str = "PodSchedulingContext";
    const NAME: &'static str = "podschedulingcontexts";
}
impl crate::HasMetadata for PodSchedulingContext {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for PodSchedulingContext {
    type Spec = crate::api::resource::v1alpha2::PodSchedulingContextSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for PodSchedulingContext {
    type Status = crate::api::resource::v1alpha2::PodSchedulingContextStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}


impl crate::Resource for ResourceClaim {
    const API_VERSION: &'static str = "resource.k8s.io/v1alpha2";
    const GROUP: &'static str = "resource.k8s.io";
    const VERSION: &'static str = "v1alpha2";
    const KIND: &'static str = "ResourceClaim";
    const NAME: &'static str = "resourceclaims";
}
impl crate::HasMetadata for ResourceClaim {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for ResourceClaim {
    type Spec = crate::api::resource::v1alpha2::ResourceClaimSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for ResourceClaim {
    type Status = crate::api::resource::v1alpha2::ResourceClaimStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}


impl crate::Resource for ResourceClaimTemplate {
    const API_VERSION: &'static str = "resource.k8s.io/v1alpha2";
    const GROUP: &'static str = "resource.k8s.io";
    const VERSION: &'static str = "v1alpha2";
    const KIND: &'static str = "ResourceClaimTemplate";
    const NAME: &'static str = "resourceclaimtemplates";
}
impl crate::HasMetadata for ResourceClaimTemplate {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for ResourceClaimTemplate {
    type Spec = crate::api::resource::v1alpha2::ResourceClaimTemplateSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}


impl crate::Resource for ResourceClass {
    const API_VERSION: &'static str = "resource.k8s.io/v1alpha2";
    const GROUP: &'static str = "resource.k8s.io";
    const VERSION: &'static str = "v1alpha2";
    const KIND: &'static str = "ResourceClass";
    const NAME: &'static str = "resourceclasses";
}
impl crate::HasMetadata for ResourceClass {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}

