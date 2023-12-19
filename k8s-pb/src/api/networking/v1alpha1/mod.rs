/// IPAddress represents a single IP of a single IP Family. The object is designed to be used by APIs
/// that operate on IP addresses. The object is used by the Service core API for allocation of IP addresses.
/// An IP address can be represented in different formats, to guarantee the uniqueness of the IP,
/// the name of the object is the IP address in canonical format, four decimal digits separated
/// by dots suppressing leading zeros for IPv4 and the representation defined by RFC 5952 for IPv6.
/// Valid: 192.168.1.5 or 2001:db8::1 or 2001:db8:aaaa:bbbb:cccc:dddd:eeee:1
/// Invalid: 10.01.2.3 or 2001:db8:0:0:0::1
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddress {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// spec is the desired state of the IPAddress.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<IpAddressSpec>,
}
/// IPAddressList contains a list of IPAddress.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddressList {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items is the list of IPAddresses.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<IpAddress>,
}
/// IPAddressSpec describe the attributes in an IP Address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddressSpec {
    /// ParentRef references the resource that an IPAddress is attached to.
    /// An IPAddress must reference a parent object.
    /// +required
    #[prost(message, optional, tag = "1")]
    pub parent_ref: ::core::option::Option<ParentReference>,
}
/// ParentReference describes a reference to a parent object.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentReference {
    /// Group is the group of the object being referenced.
    /// +optional
    #[prost(string, optional, tag = "1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource is the resource of the object being referenced.
    /// +required
    #[prost(string, optional, tag = "2")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace is the namespace of the object being referenced.
    /// +optional
    #[prost(string, optional, tag = "3")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of the object being referenced.
    /// +required
    #[prost(string, optional, tag = "4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// ServiceCIDR defines a range of IP addresses using CIDR format (e.g. 192.168.0.0/24 or 2001:db2::/64).
/// This range is used to allocate ClusterIPs to Service objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCidr {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta,
    >,
    /// spec is the desired state of the ServiceCIDR.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<ServiceCidrSpec>,
    /// status represents the current state of the ServiceCIDR.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<ServiceCidrStatus>,
}
/// ServiceCIDRList contains a list of ServiceCIDR objects.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCidrList {
    /// Standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<
        super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta,
    >,
    /// items is the list of ServiceCIDRs.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<ServiceCidr>,
}
/// ServiceCIDRSpec define the CIDRs the user wants to use for allocating ClusterIPs for Services.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCidrSpec {
    /// CIDRs defines the IP blocks in CIDR notation (e.g. "192.168.0.0/24" or "2001:db8::/64")
    /// from which to assign service cluster IPs. Max of two CIDRs is allowed, one of each IP family.
    /// This field is immutable.
    /// +optional
    #[prost(string, repeated, tag = "1")]
    pub cidrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ServiceCIDRStatus describes the current state of the ServiceCIDR.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceCidrStatus {
    /// conditions holds an array of metav1.Condition that describe the state of the ServiceCIDR.
    /// Current service state
    /// +optional
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::prost::alloc::vec::Vec<
        super::super::super::apimachinery::pkg::apis::meta::v1::Condition,
    >,
}

impl crate::Resource for IpAddress {
    const API_VERSION: &'static str = "networking.k8s.io/v1alpha1";
    const GROUP: &'static str = "networking.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "IPAddress";
    const NAME: &'static str = "ipaddresses";
}
impl crate::HasMetadata for IpAddress {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for IpAddress {
    type Spec = crate::api::networking::v1alpha1::IpAddressSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}


impl crate::Resource for ServiceCidr {
    const API_VERSION: &'static str = "networking.k8s.io/v1alpha1";
    const GROUP: &'static str = "networking.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "ServiceCIDR";
    const NAME: &'static str = "servicecidrs";
}
impl crate::HasMetadata for ServiceCidr {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for ServiceCidr {
    type Spec = crate::api::networking::v1alpha1::ServiceCidrSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for ServiceCidr {
    type Status = crate::api::networking::v1alpha1::ServiceCidrStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for ServiceCidr {
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

