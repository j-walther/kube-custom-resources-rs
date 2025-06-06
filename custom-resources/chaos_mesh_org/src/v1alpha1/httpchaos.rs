// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/httpchaos.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "HTTPChaos", plural = "httpchaos")]
#[kube(namespaced)]
#[kube(status = "HTTPChaosStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct HTTPChaosSpec {
    /// Abort is a rule to abort a http session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub abort: Option<bool>,
    /// Code is a rule to select target by http status code in response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    /// Delay represents the delay of the target request/response.
    /// A duration string is a possibly unsigned sequence of
    /// decimal numbers, each with optional fraction and a unit suffix,
    /// such as "300ms", "2h45m".
    /// Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
    /// Duration represents the duration of the chaos action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Method is a rule to select target by http method in request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Mode defines the mode to run chaos action.
    /// Supported mode: one / all / fixed / fixed-percent / random-max-percent
    pub mode: HTTPChaosMode,
    /// Patch is a rule to patch some contents in target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch: Option<HTTPChaosPatch>,
    /// Path is a rule to select target by uri path in http request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port represents the target port to be proxy of.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    /// RemoteCluster represents the remote cluster where the chaos will be deployed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteCluster")]
    pub remote_cluster: Option<String>,
    /// Replace is a rule to replace some contents in target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<HTTPChaosReplace>,
    /// RequestHeaders is a rule to select target by http headers in request.
    /// The key-value pairs represent header name and header value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_headers: Option<BTreeMap<String, String>>,
    /// ResponseHeaders is a rule to select target by http headers in response.
    /// The key-value pairs represent header name and header value pairs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<BTreeMap<String, String>>,
    /// Selector is used to select pods that are used to inject chaos action.
    pub selector: HTTPChaosSelector,
    /// Target is the object to be selected and injected.
    pub target: HTTPChaosTarget,
    /// TLS is the tls config,
    /// will override PodHttpChaos if there are multiple HTTPChaos experiments are applied
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<HTTPChaosTls>,
    /// Value is required when the mode is set to `FixedMode` / `FixedPercentMode` / `RandomMaxPercentMode`.
    /// If `FixedMode`, provide an integer of pods to do chaos action.
    /// If `FixedPercentMode`, provide a number from 0-100 to specify the percent of pods the server can do chaos action.
    /// IF `RandomMaxPercentMode`,  provide a number from 0-100 to specify the max percent of pods to do chaos action
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HTTPChaosMode {
    #[serde(rename = "one")]
    One,
    #[serde(rename = "all")]
    All,
    #[serde(rename = "fixed")]
    Fixed,
    #[serde(rename = "fixed-percent")]
    FixedPercent,
    #[serde(rename = "random-max-percent")]
    RandomMaxPercent,
}

/// Patch is a rule to patch some contents in target.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosPatch {
    /// Body is a rule to patch message body of target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<HTTPChaosPatchBody>,
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
pub struct HTTPChaosPatchBody {
    /// Type represents the patch type, only support `JSON` as [merge patch json](https://tools.ietf.org/html/rfc7396) currently.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Value is the patch contents.
    pub value: String,
}

/// Replace is a rule to replace some contents in target.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosReplace {
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

/// Selector is used to select pods that are used to inject chaos action.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosSelector {
    /// Map of string keys and values that can be used to select objects.
    /// A selector based on annotations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "annotationSelectors")]
    pub annotation_selectors: Option<BTreeMap<String, String>>,
    /// a slice of label selector expressions that can be used to select objects.
    /// A list of selectors based on set-based label expressions.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expressionSelectors")]
    pub expression_selectors: Option<Vec<HTTPChaosSelectorExpressionSelectors>>,
    /// Map of string keys and values that can be used to select objects.
    /// A selector based on fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldSelectors")]
    pub field_selectors: Option<BTreeMap<String, String>>,
    /// Map of string keys and values that can be used to select objects.
    /// A selector based on labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelectors")]
    pub label_selectors: Option<BTreeMap<String, String>>,
    /// Namespaces is a set of namespace to which objects belong.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    /// Map of string keys and values that can be used to select nodes.
    /// Selector which must match a node's labels,
    /// and objects must belong to these selected nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeSelectors")]
    pub node_selectors: Option<BTreeMap<String, String>>,
    /// Nodes is a set of node name and objects must belong to these nodes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<String>>,
    /// PodPhaseSelectors is a set of condition of a pod at the current time.
    /// supported value: Pending / Running / Succeeded / Failed / Unknown
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podPhaseSelectors")]
    pub pod_phase_selectors: Option<Vec<String>>,
    /// Pods is a map of string keys and a set values that used to select pods.
    /// The key defines the namespace which pods belong,
    /// and the each values is a set of pod names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pods: Option<BTreeMap<String, String>>,
}

/// A label selector requirement is a selector that contains values, a key, and an operator that
/// relates the key and values.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosSelectorExpressionSelectors {
    /// key is the label key that the selector applies to.
    pub key: String,
    /// operator represents a key's relationship to a set of values.
    /// Valid operators are In, NotIn, Exists and DoesNotExist.
    pub operator: String,
    /// values is an array of string values. If the operator is In or NotIn,
    /// the values array must be non-empty. If the operator is Exists or DoesNotExist,
    /// the values array must be empty. This array is replaced during a strategic
    /// merge patch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HTTPChaosTarget {
    Request,
    Response,
}

/// TLS is the tls config,
/// will override PodHttpChaos if there are multiple HTTPChaos experiments are applied
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosTls {
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

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosStatus {
    /// Conditions represents the current global condition of the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<HTTPChaosStatusConditions>>,
    /// Experiment records the last experiment state.
    pub experiment: HTTPChaosStatusExperiment,
    /// Instances always specifies podhttpchaos generation or empty
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<BTreeMap<String, i64>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosStatusExperiment {
    /// Records are used to track the running status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRecords")]
    pub container_records: Option<Vec<HTTPChaosStatusExperimentContainerRecords>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredPhase")]
    pub desired_phase: Option<HTTPChaosStatusExperimentDesiredPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosStatusExperimentContainerRecords {
    /// Events are the essential details about the injections and recoveries
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<HTTPChaosStatusExperimentContainerRecordsEvents>>,
    pub id: String,
    /// InjectedCount is a counter to record the sum of successful injections
    #[serde(rename = "injectedCount")]
    pub injected_count: i64,
    pub phase: String,
    /// RecoveredCount is a counter to record the sum of successful recoveries
    #[serde(rename = "recoveredCount")]
    pub recovered_count: i64,
    #[serde(rename = "selectorKey")]
    pub selector_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HTTPChaosStatusExperimentContainerRecordsEvents {
    /// Message is the detail message, e.g. the reason why we failed to inject the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Operation represents the operation we are doing, when we crate this event
    pub operation: String,
    /// Timestamp is time when we create this event
    pub timestamp: String,
    /// Type means the stage of this event
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HTTPChaosStatusExperimentDesiredPhase {
    Run,
    Stop,
}

