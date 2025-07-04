// This file is @generated by prost-build.
/// The names of the group, the version, and the resource.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GroupVersionResource {
    /// The name of the group.
    #[prost(string, optional, tag = "1")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the version.
    #[prost(string, optional, tag = "2")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// The name of the resource.
    #[prost(string, optional, tag = "3")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
}
/// Describes the state of a migration at a certain point.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct MigrationCondition {
    /// Type of the condition.
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// Status of the condition, one of True, False, Unknown.
    #[prost(string, optional, tag = "2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// The last time this condition was updated.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub last_update_time:
        ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// The reason for the condition's last transition.
    /// +optional
    #[prost(string, optional, tag = "4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// A human readable message indicating details about the transition.
    /// +optional
    #[prost(string, optional, tag = "5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// StorageVersionMigration represents a migration of stored data to the latest
/// storage version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVersionMigration {
    /// Standard object metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Specification of the migration.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<StorageVersionMigrationSpec>,
    /// Status of the migration.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<StorageVersionMigrationStatus>,
}
/// StorageVersionMigrationList is a collection of storage version migrations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVersionMigrationList {
    /// Standard list metadata
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of StorageVersionMigration
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<StorageVersionMigration>,
}
/// Spec of the storage version migration.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct StorageVersionMigrationSpec {
    /// The resource that is being migrated. The migrator sends requests to
    /// the endpoint serving the resource.
    /// Immutable.
    #[prost(message, optional, tag = "1")]
    pub resource: ::core::option::Option<GroupVersionResource>,
    /// The token used in the list options to get the next chunk of objects
    /// to migrate. When the .status.conditions indicates the migration is
    /// "Running", users can use this token to check the progress of the
    /// migration.
    /// +optional
    #[prost(string, optional, tag = "2")]
    pub continue_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// Status of the storage version migration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageVersionMigrationStatus {
    /// The latest available observations of the migration's current state.
    /// +patchMergeKey=type
    /// +patchStrategy=merge
    /// +listType=map
    /// +listMapKey=type
    /// +optional
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::prost::alloc::vec::Vec<MigrationCondition>,
    /// ResourceVersion to compare with the GC cache for performing the migration.
    /// This is the current resource version of given group, version and resource when
    /// kube-controller-manager first observes this StorageVersionMigration resource.
    #[prost(string, optional, tag = "2")]
    pub resource_version: ::core::option::Option<::prost::alloc::string::String>,
}

impl crate::Resource for StorageVersionMigration {
    const API_VERSION: &'static str = "storagemigration.k8s.io/v1alpha1";
    const GROUP: &'static str = "storagemigration.k8s.io";
    const VERSION: &'static str = "v1alpha1";
    const KIND: &'static str = "StorageVersionMigration";
    const URL_PATH_SEGMENT: &'static str = "storageversionmigrations";
    type Scope = crate::ClusterResourceScope;
}
impl crate::Metadata for StorageVersionMigration {
    type Ty = crate::apimachinery::pkg::apis::meta::v1::ObjectMeta;
    fn metadata(&self) -> Option<&<Self as crate::Metadata>::Ty> {
        self.metadata.as_ref()
    }
    fn metadata_mut(&mut self) -> Option<&mut <Self as crate::Metadata>::Ty> {
        self.metadata.as_mut()
    }
}
impl crate::HasSpec for StorageVersionMigration {
    type Spec = crate::api::storagemigration::v1alpha1::StorageVersionMigrationSpec;
    fn spec(&self) -> Option<&<Self as crate::HasSpec>::Spec> {
        self.spec.as_ref()
    }
    fn spec_mut(&mut self) -> Option<&mut <Self as crate::HasSpec>::Spec> {
        self.spec.as_mut()
    }
}
impl crate::HasStatus for StorageVersionMigration {
    type Status = crate::api::storagemigration::v1alpha1::StorageVersionMigrationStatus;
    fn status(&self) -> Option<&<Self as crate::HasStatus>::Status> {
        self.status.as_ref()
    }
    fn status_mut(&mut self) -> Option<&mut <Self as crate::HasStatus>::Status> {
        self.status.as_mut()
    }
}
impl crate::HasConditions for StorageVersionMigration {
    type Condition = crate::api::storagemigration::v1alpha1::MigrationCondition;
    fn conditions(&self) -> Option<&[<Self as crate::HasConditions>::Condition]> {
        self.status.as_ref().map(|s| s.conditions.as_slice())
    }
    fn conditions_mut(&mut self) -> Option<&mut Vec<<Self as crate::HasConditions>::Condition>> {
        self.status.as_mut().and_then(|s| Some(s.conditions.as_mut()))
    }
}
