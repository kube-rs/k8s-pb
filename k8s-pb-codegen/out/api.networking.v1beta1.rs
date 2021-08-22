/// HTTPIngressPath associates a path with a backend. Incoming urls matching the
/// path are forwarded to the backend.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpIngressPath {
    /// Path is matched against the path of an incoming request. Currently it can
    /// contain characters disallowed from the conventional "path" part of a URL
    /// as defined by RFC 3986. Paths must begin with a '/' and must be present
    /// when using PathType with value "Exact" or "Prefix".
    /// +optional
    #[prost(string, optional, tag="1")]
    pub path: ::core::option::Option<::prost::alloc::string::String>,
    /// PathType determines the interpretation of the Path matching. PathType can
    /// be one of the following values:
    /// * Exact: Matches the URL path exactly.
    /// * Prefix: Matches based on a URL path prefix split by '/'. Matching is
    ///   done on a path element by element basis. A path element refers is the
    ///   list of labels in the path split by the '/' separator. A request is a
    ///   match for path p if every p is an element-wise prefix of p of the
    ///   request path. Note that if the last element of the path is a substring
    ///   of the last element in request path, it is not a match (e.g. /foo/bar
    ///   matches /foo/bar/baz, but does not match /foo/barbaz).
    /// * ImplementationSpecific: Interpretation of the Path matching is up to
    ///   the IngressClass. Implementations can treat this as a separate PathType
    ///   or treat it identically to Prefix or Exact path types.
    /// Implementations are required to support all path types.
    /// Defaults to ImplementationSpecific.
    #[prost(string, optional, tag="3")]
    pub path_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Backend defines the referenced service endpoint to which the traffic
    /// will be forwarded to.
    #[prost(message, optional, tag="2")]
    pub backend: ::core::option::Option<IngressBackend>,
}
/// HTTPIngressRuleValue is a list of http selectors pointing to backends.
/// In the example: http://<host>/<path>?<searchpart> -> backend where
/// where parts of the url correspond to RFC 3986, this resource will be used
/// to match against everything after the last '/' and before the first '?'
/// or '#'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpIngressRuleValue {
    /// A collection of paths that map requests to backends.
    #[prost(message, repeated, tag="1")]
    pub paths: ::prost::alloc::vec::Vec<HttpIngressPath>,
}
/// Ingress is a collection of rules that allow inbound connections to reach the
/// endpoints defined by a backend. An Ingress can be configured to give services
/// externally-reachable urls, load balance traffic, terminate SSL, offer name
/// based virtual hosting etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ingress {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec is the desired state of the Ingress.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<IngressSpec>,
    /// Status is the current state of the Ingress.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="3")]
    pub status: ::core::option::Option<IngressStatus>,
}
/// IngressBackend describes all endpoints for a given service and port.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressBackend {
    /// Specifies the name of the referenced service.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub service_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies the port of the referenced service.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub service_port: ::core::option::Option<super::super::super::apimachinery::pkg::util::intstr::IntOrString>,
    /// Resource is an ObjectRef to another Kubernetes resource in the namespace
    /// of the Ingress object. If resource is specified, serviceName and servicePort
    /// must not be specified.
    /// +optional
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<super::super::core::v1::TypedLocalObjectReference>,
}
/// IngressClass represents the class of the Ingress, referenced by the Ingress
/// Spec. The `ingressclass.kubernetes.io/is-default-class` annotation can be
/// used to indicate that an IngressClass should be considered default. When a
/// single IngressClass resource has this annotation set to true, new Ingress
/// resources without a class specified will be assigned this default class.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressClass {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
    /// Spec is the desired state of the IngressClass.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status
    /// +optional
    #[prost(message, optional, tag="2")]
    pub spec: ::core::option::Option<IngressClassSpec>,
}
/// IngressClassList is a collection of IngressClasses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressClassList {
    /// Standard list metadata.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of IngressClasses.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<IngressClass>,
}
/// IngressClassParametersReference identifies an API object. This can be used
/// to specify a cluster or namespace-scoped resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressClassParametersReference {
    /// APIGroup is the group for the resource being referenced. If APIGroup is
    /// not specified, the specified Kind must be in the core API group. For any
    /// other third-party types, APIGroup is required.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub a_pi_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Kind is the type of resource being referenced.
    #[prost(string, optional, tag="2")]
    pub kind: ::core::option::Option<::prost::alloc::string::String>,
    /// Name is the name of resource being referenced.
    #[prost(string, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Scope represents if this refers to a cluster or namespace scoped resource.
    /// This may be set to "Cluster" (default) or "Namespace".
    /// Field can be enabled with IngressClassNamespacedParams feature gate.
    /// +optional
    /// +featureGate=IngressClassNamespacedParams
    #[prost(string, optional, tag="4")]
    pub scope: ::core::option::Option<::prost::alloc::string::String>,
    /// Namespace is the namespace of the resource being referenced. This field is
    /// required when scope is set to "Namespace" and must be unset when scope is set to
    /// "Cluster".
    /// +optional
    /// +featureGate=IngressClassNamespacedParams
    #[prost(string, optional, tag="5")]
    pub namespace: ::core::option::Option<::prost::alloc::string::String>,
}
/// IngressClassSpec provides information about the class of an Ingress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressClassSpec {
    /// Controller refers to the name of the controller that should handle this
    /// class. This allows for different "flavors" that are controlled by the
    /// same controller. For example, you may have different Parameters for the
    /// same implementing controller. This should be specified as a
    /// domain-prefixed path no more than 250 characters in length, e.g.
    /// "acme.io/ingress-controller". This field is immutable.
    #[prost(string, optional, tag="1")]
    pub controller: ::core::option::Option<::prost::alloc::string::String>,
    /// Parameters is a link to a custom resource containing additional
    /// configuration for the controller. This is optional if the controller does
    /// not require extra parameters.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub parameters: ::core::option::Option<IngressClassParametersReference>,
}
/// IngressList is a collection of Ingress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressList {
    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    /// +optional
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<super::super::super::apimachinery::pkg::apis::meta::v1::ListMeta>,
    /// Items is the list of Ingress.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<Ingress>,
}
/// IngressRule represents the rules mapping the paths under a specified host to
/// the related backend services. Incoming requests are first evaluated for a host
/// match, then routed to the backend associated with the matching IngressRuleValue.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressRule {
    /// Host is the fully qualified domain name of a network host, as defined by RFC 3986.
    /// Note the following deviations from the "host" part of the
    /// URI as defined in RFC 3986:
    /// 1. IPs are not allowed. Currently an IngressRuleValue can only apply to
    ///    the IP in the Spec of the parent Ingress.
    /// 2. The `:` delimiter is not respected because ports are not allowed.
    /// 	  Currently the port of an Ingress is implicitly :80 for http and
    /// 	  :443 for https.
    /// Both these may change in the future.
    /// Incoming requests are matched against the host before the
    /// IngressRuleValue. If the host is unspecified, the Ingress routes all
    /// traffic based on the specified IngressRuleValue.
    ///
    /// Host can be "precise" which is a domain name without the terminating dot of
    /// a network host (e.g. "foo.bar.com") or "wildcard", which is a domain name
    /// prefixed with a single wildcard label (e.g. "*.foo.com").
    /// The wildcard character '*' must appear by itself as the first DNS label and
    /// matches only a single label. You cannot have a wildcard label by itself (e.g. Host == "*").
    /// Requests will be matched against the Host field in the following way:
    /// 1. If Host is precise, the request matches this rule if the http host header is equal to Host.
    /// 2. If Host is a wildcard, then the request matches this rule if the http host header
    /// is to equal to the suffix (removing the first label) of the wildcard rule.
    /// +optional
    #[prost(string, optional, tag="1")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    /// IngressRuleValue represents a rule to route requests for this IngressRule.
    /// If unspecified, the rule defaults to a http catch-all. Whether that sends
    /// just traffic matching the host to the default backend or all traffic to the
    /// default backend, is left to the controller fulfilling the Ingress. Http is
    /// currently the only supported IngressRuleValue.
    /// +optional
    #[prost(message, optional, tag="2")]
    pub ingress_rule_value: ::core::option::Option<IngressRuleValue>,
}
/// IngressRuleValue represents a rule to apply against incoming requests. If the
/// rule is satisfied, the request is routed to the specified backend. Currently
/// mixing different types of rules in a single Ingress is disallowed, so exactly
/// one of the following must be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressRuleValue {
    /// +optional
    #[prost(message, optional, tag="1")]
    pub http: ::core::option::Option<HttpIngressRuleValue>,
}
/// IngressSpec describes the Ingress the user wishes to exist.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressSpec {
    /// IngressClassName is the name of the IngressClass cluster resource. The
    /// associated IngressClass defines which controller will implement the
    /// resource. This replaces the deprecated `kubernetes.io/ingress.class`
    /// annotation. For backwards compatibility, when that annotation is set, it
    /// must be given precedence over this field. The controller may emit a
    /// warning if the field and annotation have different values.
    /// Implementations of this API should ignore Ingresses without a class
    /// specified. An IngressClass resource may be marked as default, which can
    /// be used to set a default value for this field. For more information,
    /// refer to the IngressClass documentation.
    /// +optional
    #[prost(string, optional, tag="4")]
    pub ingress_class_name: ::core::option::Option<::prost::alloc::string::String>,
    /// A default backend capable of servicing requests that don't match any
    /// rule. At least one of 'backend' or 'rules' must be specified. This field
    /// is optional to allow the loadbalancer controller or defaulting logic to
    /// specify a global default.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub backend: ::core::option::Option<IngressBackend>,
    /// TLS configuration. Currently the Ingress only supports a single TLS
    /// port, 443. If multiple members of this list specify different hosts, they
    /// will be multiplexed on the same port according to the hostname specified
    /// through the SNI TLS extension, if the ingress controller fulfilling the
    /// ingress supports SNI.
    /// +optional
    #[prost(message, repeated, tag="2")]
    pub tls: ::prost::alloc::vec::Vec<IngressTls>,
    /// A list of host rules used to configure the Ingress. If unspecified, or
    /// no rule matches, all traffic is sent to the default backend.
    /// +optional
    #[prost(message, repeated, tag="3")]
    pub rules: ::prost::alloc::vec::Vec<IngressRule>,
}
/// IngressStatus describe the current state of the Ingress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressStatus {
    /// LoadBalancer contains the current status of the load-balancer.
    /// +optional
    #[prost(message, optional, tag="1")]
    pub load_balancer: ::core::option::Option<super::super::core::v1::LoadBalancerStatus>,
}
/// IngressTLS describes the transport layer security associated with an Ingress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IngressTls {
    /// Hosts are a list of hosts included in the TLS certificate. The values in
    /// this list must match the name/s used in the tlsSecret. Defaults to the
    /// wildcard host setting for the loadbalancer controller fulfilling this
    /// Ingress, if left unspecified.
    /// +optional
    #[prost(string, repeated, tag="1")]
    pub hosts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// SecretName is the name of the secret used to terminate TLS traffic on
    /// port 443. Field is left optional to allow TLS routing based on SNI
    /// hostname alone. If the SNI host in a listener conflicts with the "Host"
    /// header field used by an IngressRule, the SNI host is used for termination
    /// and value of the Host header is used for routing.
    /// +optional
    #[prost(string, optional, tag="2")]
    pub secret_name: ::core::option::Option<::prost::alloc::string::String>,
}
