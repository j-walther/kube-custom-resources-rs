// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/podhttpchaos.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// PodHttpChaosSpec defines the desired state of PodHttpChaos.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "PodHttpChaos", plural = "podhttpchaos")]
#[kube(namespaced)]
#[kube(status = "PodHttpChaosStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PodHttpChaosSpec {
    /// Rules are a list of injection rule for http request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<PodHttpChaosRules>>,
    /// TLS is the tls config,
    /// will be override if there are multiple HTTPChaos experiments are applied
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<PodHttpChaosTls>,
}

/// PodHttpChaosRule defines the injection rule for http.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodHttpChaosRules {
    /// Actions contains rules to inject target.
    pub actions: PodHttpChaosRulesActions,
    /// Port represents the target port to be proxy of.
    pub port: i32,
    /// Selector contains the rules to select target.
    pub selector: PodHttpChaosRulesSelector,
    /// Source represents the source of current rules
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Target is the object to be selected and injected, <Request|Response>.
    pub target: String,
}

/// Actions contains rules to inject target.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodHttpChaosRulesActions {
    /// Abort is a rule to abort a http session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abort: Option<bool>,
    /// Delay represents the delay of the target request/response.
    /// A duration string is a possibly unsigned sequence of
    /// decimal numbers, each with optional fraction and a unit suffix,
    /// such as "300ms", "2h45m".
    /// Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    /// Patch is a rule to patch some contents in target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<PodHttpChaosRulesActionsPatch>,
    /// Replace is a rule to replace some contents in target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<PodHttpChaosRulesActionsReplace>,
}

/// Patch is a rule to patch some contents in target.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodHttpChaosRulesActionsPatch {
    /// Body is a rule to patch message body of target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<PodHttpChaosRulesActionsPatchBody>,
    /// Headers is a rule to append http headers of target.
    /// For example: `[["Set-Cookie", "<one cookie>"], ["Set-Cookie", "<another cookie>"]]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<String>>,
    /// Queries is a rule to append uri queries of target(Request only).
    /// For example: `[["foo", "bar"], ["foo", "unknown"]]`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<String>>,
}

/// Body is a rule to patch message body of target.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodHttpChaosRulesActionsPatchBody {
    /// Type represents the patch type, only support `JSON` as [merge patch json](https://tools.ietf.org/html/rfc7396) currently.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Value is the patch contents.
    pub value: String,
}

/// Replace is a rule to replace some contents in target.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodHttpChaosRulesActionsReplace {
    /// Body is a rule to replace http message body in target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Code is a rule to replace http status code in response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Headers is a rule to replace http headers of target.
    /// The key-value pairs represent header name and header value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<BTreeMap<String, String>>,
    /// Method is a rule to replace http method in request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Path is rule to to replace uri path in http request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Queries is a rule to replace uri queries in http request.
    /// For example, with value `{ "foo": "unknown" }`, the `/?foo=bar` will be altered to `/?foo=unknown`,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queries: Option<BTreeMap<String, String>>,
}

/// Selector contains the rules to select target.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodHttpChaosRulesSelector {
    /// Code is a rule to select target by http status code in response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Method is a rule to select target by http method in request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Path is a rule to select target by uri path in http request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port is a rule to select server listening on specific port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// RequestHeaders is a rule to select target by http headers in request.
    /// The key-value pairs represent header name and header value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<BTreeMap<String, String>>,
    /// ResponseHeaders is a rule to select target by http headers in response.
    /// The key-value pairs represent header name and header value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<BTreeMap<String, String>>,
}

/// TLS is the tls config,
/// will be override if there are multiple HTTPChaos experiments are applied
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodHttpChaosTls {
    /// CAName represents the data name of ca file in secret, `ca.crt` for example
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caName")]
    pub ca_name: Option<String>,
    /// CertName represents the data name of cert file in secret, `tls.crt` for example
    #[serde(rename = "certName")]
    pub cert_name: String,
    /// KeyName represents the data name of key file in secret, `tls.key` for example
    #[serde(rename = "keyName")]
    pub key_name: String,
    /// SecretName represents the name of required secret resource
    #[serde(rename = "secretName")]
    pub secret_name: String,
    /// SecretNamespace represents the namespace of required secret resource
    #[serde(rename = "secretNamespace")]
    pub secret_namespace: String,
}

/// PodHttpChaosStatus defines the actual state of PodHttpChaos.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PodHttpChaosStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedMessage")]
    pub failed_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// Pid represents a running tproxy process id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// StartTime represents the start time of a tproxy process.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<i64>,
}

