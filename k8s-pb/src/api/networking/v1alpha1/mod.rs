/// ClusterCIDR represents a single configuration for per-Node Pod CIDR
/// allocations when the MultiCIDRRangeAllocator is enabled (see the config for
/// kube-controller-manager).  A cluster may have any number of ClusterCIDR
/// resources, all of which will be considered when allocating a CIDR for a
/// Node.  A ClusterCIDR is eligible to be used for a given Node when the node
/// selector matches the node in question and has free CIDRs to allocate.  In
/// case of multiple matching ClusterCIDR resources, the allocator will attempt
/// to break ties using internal heuristics, but any ClusterCIDR whose node
/// selector matches the Node may be used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterCidr {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec is the desired state of the ClusterCIDR.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<ClusterCidrSpec>,
}
/// ClusterCIDRList contains a list of ClusterCIDR.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterCidrList {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of ClusterCIDRs.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<ClusterCidr>,
}
/// ClusterCIDRSpec defines the desired state of ClusterCIDR.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterCidrSpec {
    /// nodeSelector defines which nodes the config is applicable to.
    /// An empty or nil nodeSelector selects all nodes.
    /// This field is immutable.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub node_selector: ::core::option::Option<super::super::core::v1::NodeSelector>,
    /// perNodeHostBits defines the number of host bits to be configured per node.
    /// A subnet mask determines how much of the address is used for network bits
    /// and host bits. For example an IPv4 address of 192.168.0.0/24, splits the
    /// address into 24 bits for the network portion and 8 bits for the host portion.
    /// To allocate 256 IPs, set this field to 8 (a /24 mask for IPv4 or a /120 for IPv6).
    /// Minimum value is 4 (16 IPs).
    /// This field is immutable.
    /// +required
    #[prost(int32, optional, tag="2")]
    pub per_node_host_bits: ::core::option::Option<i32>,
    /// ipv4 defines an IPv4 IP block in CIDR notation(e.g. "10.0.0.0/8").
    /// At least one of ipv4 and ipv6 must be specified.
    /// This field is immutable.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub ipv4: ::core::option::Option<::prost::alloc::string::String>,
    /// ipv6 defines an IPv6 IP block in CIDR notation(e.g. "2001:db8::/64").
    /// At least one of ipv4 and ipv6 must be specified.
    /// This field is immutable.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub ipv6: ::core::option::Option<::prost::alloc::string::String>,
}
/// IPAddress represents a single IP of a single IP Family. The object is designed to be used by APIs
/// that operate on IP addresses. The object is used by the Service core API for allocation of IP addresses.
/// An IP address can be represented in different formats, to guarantee the uniqueness of the IP,
/// the name of the object is the IP address in canonical format, four decimal digits separated
/// by dots suppressing leading zeros for IPv4 and the representation defined by RFC 5952 for IPv6.
/// Valid: 192.168.1.5 or 2001:db8::1 or 2001:db8:aaaa:bbbb:cccc:dddd:eeee:1
/// Invalid: 10.01.2.3 or 2001:db8:0:0:0::1
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddress {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec is the desired state of the IPAddress.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<IpAddressSpec>,
}
/// IPAddressList contains a list of IPAddress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddressList {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is the list of IPAddresses.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<IpAddress>,
}
/// IPAddressSpec describe the attributes in an IP Address.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAddressSpec {
    /// ParentRef references the resource that an IPAddress is attached to.
    /// An IPAddress must reference a parent object.
    /// +required
    #[prost(message, optional, tag="1")]
    pub parent_ref: ::core::option::Option<ParentReference>,
}
/// ParentReference describes a reference to a parent object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentReference {
    /// Group is the group of the object being referenced.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource is the resource of the object being referenced.
    /// +required
    #[prost(string, optional, tag="2")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace is the namespace of the object being referenced.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of the object being referenced.
    /// +required
    #[prost(string, optional, tag="4")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// UID is the uid of the object being referenced.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
}

impl crate::Resource for ClusterCidr {
    const API_VERSION: &'static str = "networking.k8s.io/v1alpha1";
    const GROUP: &'static str = "networking.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "ClusterCIDR";
    const NAME: &'static str = "clustercidrs";
}
impl crate::HasMetadata for ClusterCidr {
    type Metadata = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::HasMetadata>::Metadata> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for ClusterCidr {
    type Spec = crate::api::networking::v1alpha1::ClusterCidrSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
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

