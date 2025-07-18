// This file is @generated by prost-build.
/// ExemptPriorityLevelConfiguration describes the configurable aspects
/// of the handling of exempt requests.
/// In the mandatory exempt configuration object the values in the fields
/// here can be modified by authorized users, unlike the rest of the `spec`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ExemptPriorityLevelConfiguration {
    /// `nominalConcurrencyShares` (NCS) contributes to the computation of the
    /// NominalConcurrencyLimit (NominalCL) of this level.
    /// This is the number of execution seats nominally reserved for this priority level.
    /// This DOES NOT limit the dispatching from this priority level
    /// but affects the other priority levels through the borrowing mechanism.
    /// The server's concurrency limit (ServerCL) is divided among all the
    /// priority levels in proportion to their NCS values:
    ///
    /// NominalCL(i)  = ceil( ServerCL * NCS(i) / sum_ncs )
    /// sum_ncs = sum\[priority level k\] NCS(k)
    ///
    /// Bigger numbers mean a larger nominal concurrency limit,
    /// at the expense of every other priority level.
    /// This field has a default value of zero.
    /// +optional
    #[prost(int32, optional, tag = "1")]
    pub nominal_concurrency_shares: ::core::option::Option<i32>,
    /// `lendablePercent` prescribes the fraction of the level's NominalCL that
    /// can be borrowed by other priority levels.  This value of this
    /// field must be between 0 and 100, inclusive, and it defaults to 0.
    /// The number of seats that other levels can borrow from this level, known
    /// as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.
    ///
    /// LendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )
    ///
    /// +optional
    #[prost(int32, optional, tag = "2")]
    pub lendable_percent: ::core::option::Option<i32>,
}
/// FlowDistinguisherMethod specifies the method of a flow distinguisher.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FlowDistinguisherMethod {
    /// `type` is the type of flow distinguisher method
    /// The supported types are "ByUser" and "ByNamespace".
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
}
/// FlowSchema defines the schema of a group of flows. Note that a flow is made up of a set of inbound API requests with
/// similar attributes and is identified by a pair of strings: the name of the FlowSchema and a "flow distinguisher".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowSchema {
    /// `metadata` is the standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// `spec` is the specification of the desired behavior of a FlowSchema.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<FlowSchemaSpec>,
    /// `status` is the current status of a FlowSchema.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<FlowSchemaStatus>,
}
/// FlowSchemaCondition describes conditions for a FlowSchema.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FlowSchemaCondition {
    /// `type` is the type of the condition.
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// `status` is the status of the condition.
    /// Can be True, False, Unknown.
    /// Required.
    #[prost(string, optional, tag = "2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// `lastTransitionTime` is the last time the condition transitioned from one status to another.
    #[prost(message, optional, tag = "3")]
    pub last_transition_time:
        ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// `reason` is a unique, one-word, CamelCase reason for the condition's last transition.
    #[prost(string, optional, tag = "4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// `message` is a human-readable message indicating details about last transition.
    #[prost(string, optional, tag = "5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// FlowSchemaList is a list of FlowSchema objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowSchemaList {
    /// `metadata` is the standard list metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// `items` is a list of FlowSchemas.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<FlowSchema>,
}
/// FlowSchemaSpec describes how the FlowSchema's specification looks like.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowSchemaSpec {
    /// `priorityLevelConfiguration` should reference a PriorityLevelConfiguration in the cluster. If the reference cannot
    /// be resolved, the FlowSchema will be ignored and marked as invalid in its status.
    /// Required.
    #[prost(message, optional, tag = "1")]
    pub priority_level_configuration: ::core::option::Option<PriorityLevelConfigurationReference>,
    /// `matchingPrecedence` is used to choose among the FlowSchemas that match a given request. The chosen
    /// FlowSchema is among those with the numerically lowest (which we take to be logically highest)
    /// MatchingPrecedence.  Each MatchingPrecedence value must be ranged in \[1,10000\].
    /// Note that if the precedence is not specified, it will be set to 1000 as default.
    /// +optional
    #[prost(int32, optional, tag = "2")]
    pub matching_precedence: ::core::option::Option<i32>,
    /// `distinguisherMethod` defines how to compute the flow distinguisher for requests that match this schema.
    /// `nil` specifies that the distinguisher is disabled and thus will always be the empty string.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub distinguisher_method: ::core::option::Option<FlowDistinguisherMethod>,
    /// `rules` describes which requests will match this flow schema. This FlowSchema matches a request if and only if
    /// at least one member of rules matches the request.
    /// if it is an empty slice, there will be no requests matching the FlowSchema.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag = "4")]
    pub rules: ::prost::alloc::vec::Vec<PolicyRulesWithSubjects>,
}
/// FlowSchemaStatus represents the current state of a FlowSchema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowSchemaStatus {
    /// `conditions` is a list of the current states of FlowSchema.
    /// +listType=map
    /// +listMapKey=type
    /// +optional
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::prost::alloc::vec::Vec<FlowSchemaCondition>,
}
/// GroupSubject holds detailed information for group-kind subject.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GroupSubject {
    /// name is the user group that matches, or "*" to match all user groups.
    /// See <https://github.com/kubernetes/apiserver/blob/master/pkg/authentication/user/user.go> for some
    /// well-known group names.
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// LimitResponse defines how to handle requests that can not be executed right now.
/// +union
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct LimitResponse {
    /// `type` is "Queue" or "Reject".
    /// "Queue" means that requests that can not be executed upon arrival
    /// are held in a queue until they can be executed or a queuing limit
    /// is reached.
    /// "Reject" means that requests that can not be executed upon arrival
    /// are rejected.
    /// Required.
    /// +unionDiscriminator
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// `queuing` holds the configuration parameters for queuing.
    /// This field may be non-empty only if `type` is `"Queue"`.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub queuing: ::core::option::Option<QueuingConfiguration>,
}
/// LimitedPriorityLevelConfiguration specifies how to handle requests that are subject to limits.
/// It addresses two issues:
///    - How are requests for this priority level limited?
///    - What should be done with requests that exceed the limit?
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct LimitedPriorityLevelConfiguration {
    /// `assuredConcurrencyShares` (ACS) configures the execution
    /// limit, which is a limit on the number of requests of this
    /// priority level that may be executing at a given time.  ACS must
    /// be a positive number. The server's concurrency limit (SCL) is
    /// divided among the concurrency-controlled priority levels in
    /// proportion to their assured concurrency shares. This produces
    /// the assured concurrency value (ACV) --- the number of requests
    /// that may be executing at a time --- for each such priority
    /// level:
    ///
    ///              ACV(l) = ceil( SCL * ACS(l) / ( sum\[priority levels k\] ACS(k) ) )
    ///
    /// bigger numbers of ACS mean more reserved concurrent requests (at the
    /// expense of every other PL).
    /// This field has a default value of 30.
    /// +optional
    #[prost(int32, optional, tag = "1")]
    pub assured_concurrency_shares: ::core::option::Option<i32>,
    /// `limitResponse` indicates what to do with requests that can not be executed right now
    #[prost(message, optional, tag = "2")]
    pub limit_response: ::core::option::Option<LimitResponse>,
    /// `lendablePercent` prescribes the fraction of the level's NominalCL that
    /// can be borrowed by other priority levels. The value of this
    /// field must be between 0 and 100, inclusive, and it defaults to 0.
    /// The number of seats that other levels can borrow from this level, known
    /// as this level's LendableConcurrencyLimit (LendableCL), is defined as follows.
    ///
    /// LendableCL(i) = round( NominalCL(i) * lendablePercent(i)/100.0 )
    ///
    /// +optional
    #[prost(int32, optional, tag = "3")]
    pub lendable_percent: ::core::option::Option<i32>,
    /// `borrowingLimitPercent`, if present, configures a limit on how many
    /// seats this priority level can borrow from other priority levels.
    /// The limit is known as this level's BorrowingConcurrencyLimit
    /// (BorrowingCL) and is a limit on the total number of seats that this
    /// level may borrow at any one time.
    /// This field holds the ratio of that limit to the level's nominal
    /// concurrency limit. When this field is non-nil, it must hold a
    /// non-negative integer and the limit is calculated as follows.
    ///
    /// BorrowingCL(i) = round( NominalCL(i) * borrowingLimitPercent(i)/100.0 )
    ///
    /// The value of this field can be more than 100, implying that this
    /// priority level can borrow a number of seats that is greater than
    /// its own nominal concurrency limit (NominalCL).
    /// When this field is left `nil`, the limit is effectively infinite.
    /// +optional
    #[prost(int32, optional, tag = "4")]
    pub borrowing_limit_percent: ::core::option::Option<i32>,
}
/// NonResourcePolicyRule is a predicate that matches non-resource requests according to their verb and the
/// target non-resource URL. A NonResourcePolicyRule matches a request if and only if both (a) at least one member
/// of verbs matches the request and (b) at least one member of nonResourceURLs matches the request.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct NonResourcePolicyRule {
    /// `verbs` is a list of matching verbs and may not be empty.
    /// "*" matches all verbs. If it is present, it must be the only entry.
    /// +listType=set
    /// Required.
    #[prost(string, repeated, tag = "1")]
    pub verbs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// `nonResourceURLs` is a set of url prefixes that a user should have access to and may not be empty.
    /// For example:
    ///    - "/healthz" is legal
    ///    - "/hea*" is illegal
    ///    - "/hea" is legal but matches nothing
    ///    - "/hea/*" also matches nothing
    ///    - "/healthz/*" matches all per-component health checks.
    /// "*" matches all non-resource urls. if it is present, it must be the only entry.
    /// +listType=set
    /// Required.
    #[prost(string, repeated, tag = "6")]
    pub non_resource_ur_ls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PolicyRulesWithSubjects prescribes a test that applies to a request to an apiserver. The test considers the subject
/// making the request, the verb being requested, and the resource to be acted upon. This PolicyRulesWithSubjects matches
/// a request if and only if both (a) at least one member of subjects matches the request and (b) at least one member
/// of resourceRules or nonResourceRules matches the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyRulesWithSubjects {
    /// subjects is the list of normal user, serviceaccount, or group that this rule cares about.
    /// There must be at least one member in this slice.
    /// A slice that includes both the system:authenticated and system:unauthenticated user groups matches every request.
    /// +listType=atomic
    /// Required.
    #[prost(message, repeated, tag = "1")]
    pub subjects: ::prost::alloc::vec::Vec<Subject>,
    /// `resourceRules` is a slice of ResourcePolicyRules that identify matching requests according to their verb and the
    /// target resource.
    /// At least one of `resourceRules` and `nonResourceRules` has to be non-empty.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag = "2")]
    pub resource_rules: ::prost::alloc::vec::Vec<ResourcePolicyRule>,
    /// `nonResourceRules` is a list of NonResourcePolicyRules that identify matching requests according to their verb
    /// and the target non-resource URL.
    /// +listType=atomic
    /// +optional
    #[prost(message, repeated, tag = "3")]
    pub non_resource_rules: ::prost::alloc::vec::Vec<NonResourcePolicyRule>,
}
/// PriorityLevelConfiguration represents the configuration of a priority level.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriorityLevelConfiguration {
    /// `metadata` is the standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// `spec` is the specification of the desired behavior of a "request-priority".
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub spec: ::core::option::Option<PriorityLevelConfigurationSpec>,
    /// `status` is the current status of a "request-priority".
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status>
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub status: ::core::option::Option<PriorityLevelConfigurationStatus>,
}
/// PriorityLevelConfigurationCondition defines the condition of priority level.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct PriorityLevelConfigurationCondition {
    /// `type` is the type of the condition.
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// `status` is the status of the condition.
    /// Can be True, False, Unknown.
    /// Required.
    #[prost(string, optional, tag = "2")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    /// `lastTransitionTime` is the last time the condition transitioned from one status to another.
    #[prost(message, optional, tag = "3")]
    pub last_transition_time:
        ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// `reason` is a unique, one-word, CamelCase reason for the condition's last transition.
    #[prost(string, optional, tag = "4")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// `message` is a human-readable message indicating details about last transition.
    #[prost(string, optional, tag = "5")]
    pub message: ::core::option::Option<::prost::alloc::string::String>,
}
/// PriorityLevelConfigurationList is a list of PriorityLevelConfiguration objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriorityLevelConfigurationList {
    /// `metadata` is the standard object's metadata.
    /// More info: <https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata>
    /// +optional
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// `items` is a list of request-priorities.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<PriorityLevelConfiguration>,
}
/// PriorityLevelConfigurationReference contains information that points to the "request-priority" being used.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct PriorityLevelConfigurationReference {
    /// `name` is the name of the priority level configuration being referenced
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// PriorityLevelConfigurationSpec specifies the configuration of a priority level.
/// +union
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct PriorityLevelConfigurationSpec {
    /// `type` indicates whether this priority level is subject to
    /// limitation on request execution.  A value of `"Exempt"` means
    /// that requests of this priority level are not subject to a limit
    /// (and thus are never queued) and do not detract from the
    /// capacity made available to other priority levels.  A value of
    /// `"Limited"` means that (a) requests of this priority level
    /// _are_ subject to limits and (b) some of the server's limited
    /// capacity is made available exclusively to this priority level.
    /// Required.
    /// +unionDiscriminator
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// `limited` specifies how requests are handled for a Limited priority level.
    /// This field must be non-empty if and only if `type` is `"Limited"`.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub limited: ::core::option::Option<LimitedPriorityLevelConfiguration>,
    /// `exempt` specifies how requests are handled for an exempt priority level.
    /// This field MUST be empty if `type` is `"Limited"`.
    /// This field MAY be non-empty if `type` is `"Exempt"`.
    /// If empty and `type` is `"Exempt"` then the default values
    /// for `ExemptPriorityLevelConfiguration` apply.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub exempt: ::core::option::Option<ExemptPriorityLevelConfiguration>,
}
/// PriorityLevelConfigurationStatus represents the current state of a "request-priority".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriorityLevelConfigurationStatus {
    /// `conditions` is the current state of "request-priority".
    /// +listType=map
    /// +listMapKey=type
    /// +optional
    #[prost(message, repeated, tag = "1")]
    pub conditions: ::prost::alloc::vec::Vec<PriorityLevelConfigurationCondition>,
}
/// QueuingConfiguration holds the configuration parameters for queuing
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct QueuingConfiguration {
    /// `queues` is the number of queues for this priority level. The
    /// queues exist independently at each apiserver. The value must be
    /// positive.  Setting it to 1 effectively precludes
    /// shufflesharding and thus makes the distinguisher method of
    /// associated flow schemas irrelevant.  This field has a default
    /// value of 64.
    /// +optional
    #[prost(int32, optional, tag = "1")]
    pub queues: ::core::option::Option<i32>,
    /// `handSize` is a small positive number that configures the
    /// shuffle sharding of requests into queues.  When enqueuing a request
    /// at this priority level the request's flow identifier (a string
    /// pair) is hashed and the hash value is used to shuffle the list
    /// of queues and deal a hand of the size specified here.  The
    /// request is put into one of the shortest queues in that hand.
    /// `handSize` must be no larger than `queues`, and should be
    /// significantly smaller (so that a few heavy flows do not
    /// saturate most of the queues).  See the user-facing
    /// documentation for more extensive guidance on setting this
    /// field.  This field has a default value of 8.
    /// +optional
    #[prost(int32, optional, tag = "2")]
    pub hand_size: ::core::option::Option<i32>,
    /// `queueLengthLimit` is the maximum number of requests allowed to
    /// be waiting in a given queue of this priority level at a time;
    /// excess requests are rejected.  This value must be positive.  If
    /// not specified, it will be defaulted to 50.
    /// +optional
    #[prost(int32, optional, tag = "3")]
    pub queue_length_limit: ::core::option::Option<i32>,
}
/// ResourcePolicyRule is a predicate that matches some resource
/// requests, testing the request's verb and the target resource. A
/// ResourcePolicyRule matches a resource request if and only if: (a)
/// at least one member of verbs matches the request, (b) at least one
/// member of apiGroups matches the request, (c) at least one member of
/// resources matches the request, and (d) either (d1) the request does
/// not specify a namespace (i.e., `Namespace==""`) and clusterScope is
/// true or (d2) the request specifies a namespace and least one member
/// of namespaces matches the request's namespace.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ResourcePolicyRule {
    /// `verbs` is a list of matching verbs and may not be empty.
    /// "*" matches all verbs and, if present, must be the only entry.
    /// +listType=set
    /// Required.
    #[prost(string, repeated, tag = "1")]
    pub verbs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// `apiGroups` is a list of matching API groups and may not be empty.
    /// "*" matches all API groups and, if present, must be the only entry.
    /// +listType=set
    /// Required.
    #[prost(string, repeated, tag = "2")]
    pub api_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// `resources` is a list of matching resources (i.e., lowercase
    /// and plural) with, if desired, subresource.  For example, [
    /// "services", "nodes/status" ].  This list may not be empty.
    /// "*" matches all resources and, if present, must be the only entry.
    /// Required.
    /// +listType=set
    #[prost(string, repeated, tag = "3")]
    pub resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// `clusterScope` indicates whether to match requests that do not
    /// specify a namespace (which happens either because the resource
    /// is not namespaced or the request targets all namespaces).
    /// If this field is omitted or false then the `namespaces` field
    /// must contain a non-empty list.
    /// +optional
    #[prost(bool, optional, tag = "4")]
    pub cluster_scope: ::core::option::Option<bool>,
    /// `namespaces` is a list of target namespaces that restricts
    /// matches.  A request that specifies a target namespace matches
    /// only if either (a) this list contains that target namespace or
    /// (b) this list contains "*".  Note that "*" matches any
    /// specified namespace but does not match a request that _does
    /// not specify_ a namespace (see the `clusterScope` field for
    /// that).
    /// This list may be empty, but only if `clusterScope` is true.
    /// +optional
    /// +listType=set
    #[prost(string, repeated, tag = "5")]
    pub namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ServiceAccountSubject holds detailed information for service-account-kind subject.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ServiceAccountSubject {
    /// `namespace` is the namespace of matching ServiceAccount objects.
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
    /// `name` is the name of matching ServiceAccount objects, or "*" to match regardless of name.
    /// Required.
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Subject matches the originator of a request, as identified by the request authentication system. There are three
/// ways of matching an originator; by user, group, or service account.
/// +union
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Subject {
    /// `kind` indicates which one of the other fields is non-empty.
    /// Required
    /// +unionDiscriminator
    #[prost(string, optional, tag = "1")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// `user` matches based on username.
    /// +optional
    #[prost(message, optional, tag = "2")]
    pub user: ::core::option::Option<UserSubject>,
    /// `group` matches based on user group name.
    /// +optional
    #[prost(message, optional, tag = "3")]
    pub group: ::core::option::Option<GroupSubject>,
    /// `serviceAccount` matches ServiceAccounts.
    /// +optional
    #[prost(message, optional, tag = "4")]
    pub service_account: ::core::option::Option<ServiceAccountSubject>,
}
/// UserSubject holds detailed information for user-kind subject.
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct UserSubject {
    /// `name` is the username that matches, or "*" to match all usernames.
    /// Required.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
