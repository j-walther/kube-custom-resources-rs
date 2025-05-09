// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshhttproutes.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// Spec is the specification of the Kuma MeshHTTPRoute resource.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshHTTPRoute", plural = "meshhttproutes")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MeshHTTPRouteSpec {
    /// TargetRef is a reference to the resource the policy takes an effect on.
    /// The resource could be either a real store object or virtual resource
    /// defined inplace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRef")]
    pub target_ref: Option<MeshHTTPRouteTargetRef>,
    /// To matches destination services of requests and holds configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MeshHTTPRouteTo>>,
}

/// TargetRef is a reference to the resource the policy takes an effect on.
/// The resource could be either a real store object or virtual resource
/// defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteTargetRef {
    /// Kind of the referenced resource
    pub kind: MeshHTTPRouteTargetRefKind,
    /// Labels are used to select group of MeshServices that match labels. Either Labels or
    /// Name and Namespace can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace of target resource. If empty only resources in policy namespace
    /// will be targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// SectionName is used to target specific section of resource.
    /// For example, you can target port from MeshService.ports[] by its name. Only traffic to this port will be affected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource the policy takes an effect on.
/// The resource could be either a real store object or virtual resource
/// defined inplace.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshExternalService,
    MeshMultiZoneService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
    Dataplane,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteTo {
    /// Hostnames is only valid when targeting MeshGateway and limits the
    /// effects of the rules to requests to this hostname.
    /// Given hostnames must intersect with the hostname of the listeners the
    /// route attaches to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
    /// Rules contains the routing rules applies to a combination of top-level
    /// targetRef and the targetRef in this entry.
    pub rules: Vec<MeshHTTPRouteToRules>,
    /// TargetRef is a reference to the resource that represents a group of
    /// request destinations.
    #[serde(rename = "targetRef")]
    pub target_ref: MeshHTTPRouteToTargetRef,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRules {
    /// Default holds routing rules that can be merged with rules from other
    /// policies.
    pub default: MeshHTTPRouteToRulesDefault,
    /// Matches describes how to match HTTP requests this rule should be applied
    /// to.
    pub matches: Vec<MeshHTTPRouteToRulesMatches>,
}

/// Default holds routing rules that can be merged with rules from other
/// policies.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefault {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendRefs")]
    pub backend_refs: Option<Vec<MeshHTTPRouteToRulesDefaultBackendRefs>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<MeshHTTPRouteToRulesDefaultFilters>>,
}

/// BackendRef defines where to forward traffic.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultBackendRefs {
    /// Kind of the referenced resource
    pub kind: MeshHTTPRouteToRulesDefaultBackendRefsKind,
    /// Labels are used to select group of MeshServices that match labels. Either Labels or
    /// Name and Namespace can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace of target resource. If empty only resources in policy namespace
    /// will be targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port is only supported when this ref refers to a real MeshService object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// SectionName is used to target specific section of resource.
    /// For example, you can target port from MeshService.ports[] by its name. Only traffic to this port will be affected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// BackendRef defines where to forward traffic.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesDefaultBackendRefsKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshExternalService,
    MeshMultiZoneService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
    Dataplane,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFilters {
    /// Only one action is supported per header name.
    /// Configuration to set or add multiple values for a header must use RFC 7230
    /// header value formatting, separating each value with a comma.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaderModifier")]
    pub request_header_modifier: Option<MeshHTTPRouteToRulesDefaultFiltersRequestHeaderModifier>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestMirror")]
    pub request_mirror: Option<MeshHTTPRouteToRulesDefaultFiltersRequestMirror>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestRedirect")]
    pub request_redirect: Option<MeshHTTPRouteToRulesDefaultFiltersRequestRedirect>,
    /// Only one action is supported per header name.
    /// Configuration to set or add multiple values for a header must use RFC 7230
    /// header value formatting, separating each value with a comma.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaderModifier")]
    pub response_header_modifier: Option<MeshHTTPRouteToRulesDefaultFiltersResponseHeaderModifier>,
    #[serde(rename = "type")]
    pub r#type: MeshHTTPRouteToRulesDefaultFiltersType,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "urlRewrite")]
    pub url_rewrite: Option<MeshHTTPRouteToRulesDefaultFiltersUrlRewrite>,
}

/// Only one action is supported per header name.
/// Configuration to set or add multiple values for a header must use RFC 7230
/// header value formatting, separating each value with a comma.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersRequestHeaderModifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<MeshHTTPRouteToRulesDefaultFiltersRequestHeaderModifierAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<MeshHTTPRouteToRulesDefaultFiltersRequestHeaderModifierSet>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersRequestHeaderModifierAdd {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersRequestHeaderModifierSet {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersRequestMirror {
    /// BackendRef defines where to forward traffic.
    #[serde(rename = "backendRef")]
    pub backend_ref: MeshHTTPRouteToRulesDefaultFiltersRequestMirrorBackendRef,
    /// Percentage of requests to mirror. If not specified, all requests
    /// to the target cluster will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage: Option<IntOrString>,
}

/// BackendRef defines where to forward traffic.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersRequestMirrorBackendRef {
    /// Kind of the referenced resource
    pub kind: MeshHTTPRouteToRulesDefaultFiltersRequestMirrorBackendRefKind,
    /// Labels are used to select group of MeshServices that match labels. Either Labels or
    /// Name and Namespace can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace of target resource. If empty only resources in policy namespace
    /// will be targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Port is only supported when this ref refers to a real MeshService object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// SectionName is used to target specific section of resource.
    /// For example, you can target port from MeshService.ports[] by its name. Only traffic to this port will be affected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

/// BackendRef defines where to forward traffic.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesDefaultFiltersRequestMirrorBackendRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshExternalService,
    MeshMultiZoneService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
    Dataplane,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersRequestRedirect {
    /// PreciseHostname is the fully qualified domain name of a network host. This
    /// matches the RFC 1123 definition of a hostname with 1 notable exception that
    /// numeric IP addresses are not allowed.
    /// 
    /// Note that as per RFC1035 and RFC1123, a *label* must consist of lower case
    /// alphanumeric characters or '-', and must start and end with an alphanumeric
    /// character. No other punctuation is allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Path defines parameters used to modify the path of the incoming request.
    /// The modified path is then used to construct the location header.
    /// When empty, the request path is used as-is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<MeshHTTPRouteToRulesDefaultFiltersRequestRedirectPath>,
    /// Port is the port to be used in the value of the `Location`
    /// header in the response.
    /// When empty, port (if specified) of the request is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<MeshHTTPRouteToRulesDefaultFiltersRequestRedirectScheme>,
    /// StatusCode is the HTTP status code to be used in response.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCode")]
    pub status_code: Option<i64>,
}

/// Path defines parameters used to modify the path of the incoming request.
/// The modified path is then used to construct the location header.
/// When empty, the request path is used as-is.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersRequestRedirectPath {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replaceFullPath")]
    pub replace_full_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replacePrefixMatch")]
    pub replace_prefix_match: Option<String>,
    #[serde(rename = "type")]
    pub r#type: MeshHTTPRouteToRulesDefaultFiltersRequestRedirectPathType,
}

/// Path defines parameters used to modify the path of the incoming request.
/// The modified path is then used to construct the location header.
/// When empty, the request path is used as-is.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesDefaultFiltersRequestRedirectPathType {
    ReplaceFullPath,
    ReplacePrefixMatch,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesDefaultFiltersRequestRedirectScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesDefaultFiltersRequestRedirectStatusCode {
    #[serde(rename = "301")]
    r#_301,
    #[serde(rename = "302")]
    r#_302,
    #[serde(rename = "303")]
    r#_303,
    #[serde(rename = "307")]
    r#_307,
    #[serde(rename = "308")]
    r#_308,
}

/// Only one action is supported per header name.
/// Configuration to set or add multiple values for a header must use RFC 7230
/// header value formatting, separating each value with a comma.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersResponseHeaderModifier {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<MeshHTTPRouteToRulesDefaultFiltersResponseHeaderModifierAdd>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<MeshHTTPRouteToRulesDefaultFiltersResponseHeaderModifierSet>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersResponseHeaderModifierAdd {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersResponseHeaderModifierSet {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesDefaultFiltersType {
    RequestHeaderModifier,
    ResponseHeaderModifier,
    RequestRedirect,
    #[serde(rename = "URLRewrite")]
    UrlRewrite,
    RequestMirror,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersUrlRewrite {
    /// HostToBackendHostname rewrites the hostname to the hostname of the
    /// upstream host. This option is only available when targeting MeshGateways.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostToBackendHostname")]
    pub host_to_backend_hostname: Option<bool>,
    /// Hostname is the value to be used to replace the host header value during forwarding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Path defines a path rewrite.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<MeshHTTPRouteToRulesDefaultFiltersUrlRewritePath>,
}

/// Path defines a path rewrite.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToRulesDefaultFiltersUrlRewritePath {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replaceFullPath")]
    pub replace_full_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replacePrefixMatch")]
    pub replace_prefix_match: Option<String>,
    #[serde(rename = "type")]
    pub r#type: MeshHTTPRouteToRulesDefaultFiltersUrlRewritePathType,
}

/// Path defines a path rewrite.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesDefaultFiltersUrlRewritePathType {
    ReplaceFullPath,
    ReplacePrefixMatch,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesMatches {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<MeshHTTPRouteToRulesMatchesHeaders>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<MeshHTTPRouteToRulesMatchesMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<MeshHTTPRouteToRulesMatchesPath>,
    /// QueryParams matches based on HTTP URL query parameters. Multiple matches
    /// are ANDed together such that all listed matches must succeed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryParams")]
    pub query_params: Option<Vec<MeshHTTPRouteToRulesMatchesQueryParams>>,
}

/// HeaderMatch describes how to select an HTTP route by matching HTTP request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshHTTPRouteToRulesMatchesHeaders {
    /// Name is the name of the HTTP Header to be matched. Name MUST be lower case
    /// as they will be handled with case insensitivity (See https://tools.ietf.org/html/rfc7230#section-3.2).
    pub name: String,
    /// Type specifies how to match against the value of the header.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<MeshHTTPRouteToRulesMatchesHeadersType>,
    /// Value is the value of HTTP Header to be matched.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// HeaderMatch describes how to select an HTTP route by matching HTTP request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesMatchesHeadersType {
    Exact,
    Present,
    RegularExpression,
    Absent,
    Prefix,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesMatchesMethod {
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "TRACE")]
    Trace,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToRulesMatchesPath {
    #[serde(rename = "type")]
    pub r#type: MeshHTTPRouteToRulesMatchesPathType,
    /// Exact or prefix matches must be an absolute path. A prefix matches only
    /// if separated by a slash or the entire path.
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesMatchesPathType {
    Exact,
    PathPrefix,
    RegularExpression,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToRulesMatchesQueryParams {
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: MeshHTTPRouteToRulesMatchesQueryParamsType,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToRulesMatchesQueryParamsType {
    Exact,
    RegularExpression,
}

/// TargetRef is a reference to the resource that represents a group of
/// request destinations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshHTTPRouteToTargetRef {
    /// Kind of the referenced resource
    pub kind: MeshHTTPRouteToTargetRefKind,
    /// Labels are used to select group of MeshServices that match labels. Either Labels or
    /// Name and Namespace can be used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
    /// Mesh is reserved for future use to identify cross mesh resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh: Option<String>,
    /// Name of the referenced resource. Can only be used with kinds: `MeshService`,
    /// `MeshServiceSubset` and `MeshGatewayRoute`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace specifies the namespace of target resource. If empty only resources in policy namespace
    /// will be targeted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// ProxyTypes specifies the data plane types that are subject to the policy. When not specified,
    /// all data plane types are targeted by the policy.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyTypes")]
    pub proxy_types: Option<Vec<String>>,
    /// SectionName is used to target specific section of resource.
    /// For example, you can target port from MeshService.ports[] by its name. Only traffic to this port will be affected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<String>,
    /// Tags used to select a subset of proxies by tags. Can only be used with kinds
    /// `MeshSubset` and `MeshServiceSubset`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// TargetRef is a reference to the resource that represents a group of
/// request destinations.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshHTTPRouteToTargetRefKind {
    Mesh,
    MeshSubset,
    MeshGateway,
    MeshService,
    MeshExternalService,
    MeshMultiZoneService,
    MeshServiceSubset,
    #[serde(rename = "MeshHTTPRoute")]
    MeshHttpRoute,
    Dataplane,
}

