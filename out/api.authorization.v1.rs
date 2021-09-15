/// ExtraValue masks the value so protobuf can generate
/// +protobuf.nullable=true
/// +protobuf.options.(gogoproto.goproto_stringer)=false
///
/// items, if empty, will result in an empty slice
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtraValue {
    #[prost(string, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// LocalSubjectAccessReview checks whether or not a user or group can perform an action in a given namespace.
/// Having a namespace scoped resource makes it much easier to grant namespace scoped policy that includes permissions
/// checking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalSubjectAccessReview {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec holds information about the request being evaluated.  spec.namespace must be equal to the namespace
    /// you made the request against.  If empty, it is defaulted.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<SubjectAccessReviewSpec>,
    /// Status is filled in by the server and indicates whether the request is allowed or not
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<SubjectAccessReviewStatus>,
}
/// NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonResourceAttributes {
    /// Path is the URL path of the request
    /// +optional
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// Verb is the standard HTTP verb
    /// +optional
    #[prost(string, optional, tag="2")]
    pub verb: ::core::option::Option<::prost::alloc::string::String>,
}
/// NonResourceRule holds information that describes a rule for the non-resource
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NonResourceRule {
    /// Verb is a list of kubernetes non-resource API verbs, like: get, post, put, delete, patch, head, options.  "*" means all.
    #[prost(string, repeated, tag="1")]
    pub verbs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// NonResourceURLs is a set of partial urls that a user should have access to.  *s are allowed, but only as the full,
    /// final step in the path.  "*" means all.
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub non_resource_ur_ls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ResourceAttributes includes the authorization attributes available for resource requests to the Authorizer interface
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceAttributes {
    /// Namespace is the namespace of the action being requested.  Currently, there is no distinction between no namespace and all namespaces
    /// "" (empty) is defaulted for LocalSubjectAccessReviews
    /// "" (empty) is empty for cluster-scoped resources
    /// "" (empty) means "all" for namespace scoped resources from a SubjectAccessReview or SelfSubjectAccessReview
    /// +optional
    #[prost(string, optional, tag="1")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// Verb is a kubernetes resource API verb, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub verb: ::core::option::Option<::prost::alloc::string::String>,
    /// Group is the API Group of the Resource.  "*" means all.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub group: ::core::option::Option<::prost::alloc::string::String>,
    /// Version is the API Version of the Resource.  "*" means all.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource is one of the existing resource types.  "*" means all.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub resource: ::core::option::Option<::prost::alloc::string::String>,
    /// Subresource is one of the existing resource types.  "" means none.
    /// +optional
    #[prost(string, optional, tag="6")]
    pub subresource: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of the resource being requested for a "get" or deleted for a "delete". "" (empty) means all.
    /// +optional
    #[prost(string, optional, tag="7")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// ResourceRule is the list of actions the subject is allowed to perform on resources. The list ordering isn't significant,
/// may contain duplicates, and possibly be incomplete.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceRule {
    /// Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  "*" means all.
    #[prost(string, repeated, tag="1")]
    pub verbs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of
    /// the enumerated resources in any API group will be allowed.  "*" means all.
    /// +optional
    #[prost(string, repeated, tag="2")]
    pub api_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Resources is a list of resources this rule applies to.  "*" means all in the specified apiGroups.
    ///  "*/foo" represents the subresource 'foo' for all resources in the specified apiGroups.
    /// +optional
    #[prost(string, repeated, tag="3")]
    pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  "*" means all.
    /// +optional
    #[prost(string, repeated, tag="4")]
    pub resource_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// SelfSubjectAccessReview checks whether or the current user can perform an action.  Not filling in a
/// spec.namespace means "in all namespaces".  Self is a special case, because users should always be able
/// to check whether they can perform an action
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfSubjectAccessReview {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec holds information about the request being evaluated.  user and groups must be empty
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<SelfSubjectAccessReviewSpec>,
    /// Status is filled in by the server and indicates whether the request is allowed or not
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<SubjectAccessReviewStatus>,
}
/// SelfSubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes
/// and NonResourceAuthorizationAttributes must be set
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfSubjectAccessReviewSpec {
    /// ResourceAuthorizationAttributes describes information for a resource access request
    /// +optional
    #[prost(message, optional, tag="1")]
    pub resource_attributes: ::core::option::Option<ResourceAttributes>,
    /// NonResourceAttributes describes information for a non-resource access request
    /// +optional
    #[prost(message, optional, tag="2")]
    pub non_resource_attributes: ::core::option::Option<NonResourceAttributes>,
}
/// SelfSubjectRulesReview enumerates the set of actions the current user can perform within a namespace.
/// The returned list of actions may be incomplete depending on the server's authorization mode,
/// and any errors experienced during the evaluation. SelfSubjectRulesReview should be used by UIs to show/hide actions,
/// or to quickly let an end user reason about their permissions. It should NOT Be used by external systems to
/// drive authorization decisions as this raises confused deputy, cache lifetime/revocation, and correctness concerns.
/// SubjectAccessReview, and LocalAccessReview are the correct way to defer authorization decisions to the API server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfSubjectRulesReview {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec holds information about the request being evaluated.
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<SelfSubjectRulesReviewSpec>,
    /// Status is filled in by the server and indicates the set of actions a user can perform.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<SubjectRulesReviewStatus>,
}
/// SelfSubjectRulesReviewSpec defines the specification for SelfSubjectRulesReview.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SelfSubjectRulesReviewSpec {
    /// Namespace to evaluate rules for. Required.
    #[prost(string, optional, tag="1")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
}
/// SubjectAccessReview checks whether or not a user or group can perform an action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAccessReview {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec holds information about the request being evaluated
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<SubjectAccessReviewSpec>,
    /// Status is filled in by the server and indicates whether the request is allowed or not
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<SubjectAccessReviewStatus>,
}
/// SubjectAccessReviewSpec is a description of the access request.  Exactly one of ResourceAuthorizationAttributes
/// and NonResourceAuthorizationAttributes must be set
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAccessReviewSpec {
    /// ResourceAuthorizationAttributes describes information for a resource access request
    /// +optional
    #[prost(message, optional, tag="1")]
    pub resource_attributes: ::core::option::Option<ResourceAttributes>,
    /// NonResourceAttributes describes information for a non-resource access request
    /// +optional
    #[prost(message, optional, tag="2")]
    pub non_resource_attributes: ::core::option::Option<NonResourceAttributes>,
    /// User is the user you're testing for.
    /// If you specify "User" but not "Groups", then is it interpreted as "What if User were not a member of any groups
    /// +optional
    #[prost(string, optional, tag="3")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    /// Groups is the groups you're testing for.
    /// +optional
    #[prost(string, repeated, tag="4")]
    pub groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Extra corresponds to the user.Info.GetExtra() method from the authenticator.  Since that is input to the authorizer
    /// it needs a reflection here.
    /// +optional
    #[prost(map="string, message", tag="5")]
    pub extra: ::std::collections::HashMap<::prost::alloc::string::String, ExtraValue>,
    /// UID information about the requesting user.
    /// +optional
    #[prost(string, optional, tag="6")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
}
/// SubjectAccessReviewStatus
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectAccessReviewStatus {
    /// Allowed is required. True if the action would be allowed, false otherwise.
    #[prost(bool, optional, tag="1")]
    pub allowed: ::core::option::Option<bool>,
    /// Denied is optional. True if the action would be denied, otherwise
    /// false. If both allowed is false and denied is false, then the
    /// authorizer has no opinion on whether to authorize the action. Denied
    /// may not be true if Allowed is true.
    /// +optional
    #[prost(bool, optional, tag="4")]
    pub denied: ::core::option::Option<bool>,
    /// Reason is optional.  It indicates why a request was allowed or denied.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// EvaluationError is an indication that some error occurred during the authorization check.
    /// It is entirely possible to get an error and be able to continue determine authorization status in spite of it.
    /// For instance, RBAC can be missing a role, but enough roles are still present and bound to reason about the request.
    /// +optional
    #[prost(string, optional, tag="3")]
    pub evaluation_error: ::core::option::Option<::prost::alloc::string::String>,
}
/// SubjectRulesReviewStatus contains the result of a rules check. This check can be incomplete depending on
/// the set of authorizers the server is configured with and any errors experienced during evaluation.
/// Because authorization rules are additive, if a rule appears in a list it's safe to assume the subject has that permission,
/// even if that list is incomplete.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubjectRulesReviewStatus {
    /// ResourceRules is the list of actions the subject is allowed to perform on resources.
    /// The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    #[prost(message, repeated, tag="1")]
    pub resource_rules: ::prost::alloc::vec::Vec<ResourceRule>,
    /// NonResourceRules is the list of actions the subject is allowed to perform on non-resources.
    /// The list ordering isn't significant, may contain duplicates, and possibly be incomplete.
    #[prost(message, repeated, tag="2")]
    pub non_resource_rules: ::prost::alloc::vec::Vec<NonResourceRule>,
    /// Incomplete is true when the rules returned by this call are incomplete. This is most commonly
    /// encountered when an authorizer, such as an external authorizer, doesn't support rules evaluation.
    #[prost(bool, optional, tag="3")]
    pub incomplete: ::core::option::Option<bool>,
    /// EvaluationError can appear in combination with Rules. It indicates an error occurred during
    /// rule evaluation, such as an authorizer that doesn't support rule evaluation, and that
    /// ResourceRules and/or NonResourceRules may be incomplete.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub evaluation_error: ::core::option::Option<::prost::alloc::string::String>,
}
// didn't find authorization/v1