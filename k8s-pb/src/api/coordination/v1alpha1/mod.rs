// This file is @generated by prost-build.
/// LeaseCandidate defines a candidate for a Lease object.
/// Candidates are created such that coordinated leader election will pick the best leader from the list of candidates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseCandidate {
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// spec contains the specification of the Lease.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<LeaseCandidateSpec>,
}
/// LeaseCandidateList is a list of Lease objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseCandidateList {
    /// Standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is a list of schema objects.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<LeaseCandidate>,
}
/// LeaseCandidateSpec is a specification of a Lease.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeaseCandidateSpec {
    /// LeaseName is the name of the lease for which this candidate is contending.
    /// This field is immutable.
    /// +required
    #[prost(string, optional, tag = "1")]
    pub lease_name: ::core::option::Option<::prost::alloc::string::String>,
    /// PingTime is the last time that the server has requested the LeaseCandidate
    /// to renew. It is only done during leader election to check if any
    /// LeaseCandidates have become ineligible. When PingTime is updated, the
    /// LeaseCandidate will respond by updating RenewTime.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub ping_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime>,
    /// RenewTime is the time that the LeaseCandidate was last updated.
    /// Any time a Lease needs to do leader election, the PingTime field
    /// is updated to signal to the LeaseCandidate that they should update
    /// the RenewTime.
    /// Old LeaseCandidate objects are also garbage collected if it has been hours
    /// since the last renew. The PingTime field is updated regularly to prevent
    /// garbage collection for still active LeaseCandidates.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub renew_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime>,
    /// BinaryVersion is the binary version. It must be in a semver format without leading `v`.
    /// This field is required when strategy is "OldestEmulationVersion"
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub binary_version: ::core::option::Option<::prost::alloc::string::String>,
    /// EmulationVersion is the emulation version. It must be in a semver format without leading `v`.
    /// EmulationVersion must be less than or equal to BinaryVersion.
    /// This field is required when strategy is "OldestEmulationVersion"
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub emulation_version: ::core::option::Option<::prost::alloc::string::String>,
    /// PreferredStrategies indicates the list of strategies for picking the leader for coordinated leader election.
    /// The list is ordered, and the first strategy supersedes all other strategies. The list is used by coordinated
    /// leader election to make a decision about the final election strategy. This follows as
    /// - If all clients have strategy X as the first element in this list, strategy X will be used.
    /// - If a candidate has strategy \[X\] and another candidate has strategy \[Y, X\], Y supersedes X and strategy Y
    ///    will be used.
    /// - If a candidate has strategy \[X, Y\] and another candidate has strategy \[Y, X\], this is a user error and leader
    ///    election will not operate the Lease until resolved.
    /// (Alpha) Using this field requires the CoordinatedLeaderElection feature gate to be enabled.
    /// +featureGate=CoordinatedLeaderElection
    /// +listType=atomic
    /// +required
    #[prost(string, repeated, tag = "6")]
    pub preferred_strategies: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

impl crate::Resource for LeaseCandidate {
    const API_VERSION: &'static str = "coordination.k8s.io/v1alpha1";
    const GROUP: &'static str = "coordination.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "LeaseCandidate";
    const URL_PATH_SEGMENT: &'static str = "leasecandidates";
    type Scope = crate::NamespaceResourceScope;
}
impl crate::Metadata for LeaseCandidate {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> &<Self as crate::Metadata>::Ty {
        self.metadata.as_ref().unwrap()
    }
    fn metadata_mut(&mut self) -> &mut <Self as crate::Metadata>::Ty {
        self.metadata.as_mut().unwrap()
    }
}
impl crate::HasSpec for LeaseCandidate {
    type Spec = crate::api::coordination::v1alpha1::LeaseCandidateSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
