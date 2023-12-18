/// Event is a report of an event somewhere in the cluster. It generally denotes some state change in the system.
/// Events have a limited retention time and triggers and messages may evolve
/// with time.  Event consumers should not rely on the timing of an event
/// with a given Reason reflecting a consistent underlying trigger, or the
/// continued existence of events with that Reason.  Events should be
/// treated as informative, best-effort, supplemental data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// eventTime is the time when this Event was first observed. It is required.
    #[prost(message, optional, tag="2")]
    pub event_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime>,
    /// series is data about the Event series this event represents or nil if it's a singleton Event.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub series: ::core::option::Option<EventSeries>,
    /// reportingController is the name of the controller that emitted this Event, e.g. `kubernetes.io/kubelet`.
    /// This field cannot be empty for new Events.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub reporting_controller: ::core::option::Option<::prost::alloc::string::String>,
    /// reportingInstance is the ID of the controller instance, e.g. `kubelet-xyzf`.
    /// This field cannot be empty for new Events and it can have at most 128 characters.
    /// +optional
    #[prost(string, optional, tag="5")]
    pub reporting_instance: ::core::option::Option<::prost::alloc::string::String>,
    /// action is what action was taken/failed regarding to the regarding object. It is machine-readable.
    /// This field can have at most 128 characters.
    /// +optional
    #[prost(string, optional, tag="6")]
    pub action: ::core::option::Option<::prost::alloc::string::String>,
    /// reason is why the action was taken. It is human-readable.
    /// This field can have at most 128 characters.
    /// +optional
    #[prost(string, optional, tag="7")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
    /// regarding contains the object this Event is about. In most cases it's an Object reporting controller
    /// implements, e.g. ReplicaSetController implements ReplicaSets and this event is emitted because
    /// it acts on some changes in a ReplicaSet object.
    /// +optional
    #[prost(message, optional, tag="8")]
    pub regarding: ::core::option::Option<super::super::core::v1::ObjectReference>,
    /// related is the optional secondary object for more complex actions. E.g. when regarding object triggers
    /// a creation or deletion of related object.
    /// +optional
    #[prost(message, optional, tag="9")]
    pub related: ::core::option::Option<super::super::core::v1::ObjectReference>,
    /// note is a human-readable description of the status of this operation.
    /// Maximal length of the note is 1kB, but libraries should be prepared to
    /// handle values up to 64kB.
    /// +optional
    #[prost(string, optional, tag="10")]
    pub note: ::core::option::Option<::prost::alloc::string::String>,
    /// type is the type of this event (Normal, Warning), new types could be added in the future.
    /// It is machine-readable.
    /// +optional
    #[prost(string, optional, tag="11")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// deprecatedSource is the deprecated field assuring backward compatibility with core.v1 Event type.
    /// +optional
    #[prost(message, optional, tag="12")]
    pub deprecated_source: ::core::option::Option<super::super::core::v1::EventSource>,
    /// deprecatedFirstTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
    /// +optional
    #[prost(message, optional, tag="13")]
    pub deprecated_first_timestamp: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// deprecatedLastTimestamp is the deprecated field assuring backward compatibility with core.v1 Event type.
    /// +optional
    #[prost(message, optional, tag="14")]
    pub deprecated_last_timestamp: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::Time>,
    /// deprecatedCount is the deprecated field assuring backward compatibility with core.v1 Event type.
    /// +optional
    #[prost(int32, optional, tag="15")]
    pub deprecated_count: ::core::option::Option<i32>,
}
/// EventList is a list of Event objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventList {
    /// Standard list metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// items is a list of schema objects.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Event>,
}
/// EventSeries contain information on series of events, i.e. thing that was/is happening
/// continuously for some time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSeries {
    /// count is the number of occurrences in this series up to the last heartbeat time.
    #[prost(int32, optional, tag="1")]
    pub count: ::core::option::Option<i32>,
    /// lastObservedTime is the time when last Event from the series was seen before last heartbeat.
    #[prost(message, optional, tag="2")]
    pub last_observed_time: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::MicroTime>,
}
