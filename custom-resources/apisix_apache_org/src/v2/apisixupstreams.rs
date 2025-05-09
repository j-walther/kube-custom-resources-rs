// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/apache/apisix-ingress-controller/apisix.apache.org/v2/apisixupstreams.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apisix.apache.org", version = "v2", kind = "ApisixUpstream", plural = "apisixupstreams")]
#[kube(namespaced)]
#[kube(status = "ApisixUpstreamStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ApisixUpstreamSpec {
    /// Discovery is used to configure service discovery for upstream
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery: Option<ApisixUpstreamDiscovery>,
    /// ExternalNodes contains external nodes the Upstream should use If this field is set, the upstream will use these nodes directly without any further resolves
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalNodes")]
    pub external_nodes: Option<Vec<ApisixUpstreamExternalNodes>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<ApisixUpstreamHealthCheck>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ingressClassName")]
    pub ingress_class_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loadbalancer: Option<ApisixUpstreamLoadbalancer>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "passHost")]
    pub pass_host: Option<ApisixUpstreamPassHost>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portLevelSettings")]
    pub port_level_settings: Option<Vec<ApisixUpstreamPortLevelSettings>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<ApisixUpstreamScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subsets: Option<Vec<ApisixUpstreamSubsets>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ApisixUpstreamTimeout>,
    /// ApisixSecret describes the Kubernetes Secret name and namespace.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<ApisixUpstreamTlsSecret>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "upstreamHost")]
    pub upstream_host: Option<String>,
}

/// Discovery is used to configure service discovery for upstream
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamDiscovery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceName")]
    pub service_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// ApisixUpstreamExternalNode is the external node conf
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamExternalNodes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamHealthCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<ApisixUpstreamHealthCheckActive>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passive: Option<ApisixUpstreamHealthCheckPassive>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamHealthCheckActive {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<ApisixUpstreamHealthCheckActiveHealthy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpPath")]
    pub http_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    pub request_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "strictTLS")]
    pub strict_tls: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ApisixUpstreamHealthCheckActiveType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unhealthy: Option<ApisixUpstreamHealthCheckActiveUnhealthy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamHealthCheckActiveHealthy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpCodes")]
    pub http_codes: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successes: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamHealthCheckActiveType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "tcp")]
    Tcp,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamHealthCheckActiveUnhealthy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpCodes")]
    pub http_codes: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpFailures")]
    pub http_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpFailures")]
    pub tcp_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamHealthCheckPassive {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<ApisixUpstreamHealthCheckPassiveHealthy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ApisixUpstreamHealthCheckPassiveType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unhealthy: Option<ApisixUpstreamHealthCheckPassiveUnhealthy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamHealthCheckPassiveHealthy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpCodes")]
    pub http_codes: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successes: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamHealthCheckPassiveType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "tcp")]
    Tcp,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamHealthCheckPassiveUnhealthy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpCodes")]
    pub http_codes: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpFailures")]
    pub http_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpFailures")]
    pub tcp_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixUpstreamLoadbalancer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hashOn")]
    pub hash_on: Option<ApisixUpstreamLoadbalancerHashOn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "type")]
    pub r#type: ApisixUpstreamLoadbalancerType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamLoadbalancerHashOn {
    #[serde(rename = "vars")]
    Vars,
    #[serde(rename = "vars_combinations")]
    VarsCombinations,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "cookie")]
    Cookie,
    #[serde(rename = "consumer")]
    Consumer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamLoadbalancerType {
    #[serde(rename = "roundrobin")]
    Roundrobin,
    #[serde(rename = "chash")]
    Chash,
    #[serde(rename = "ewma")]
    Ewma,
    #[serde(rename = "least_conn")]
    LeastConn,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamPassHost {
    #[serde(rename = "pass")]
    Pass,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "rewrite")]
    Rewrite,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettings {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthCheck")]
    pub health_check: Option<ApisixUpstreamPortLevelSettingsHealthCheck>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loadbalancer: Option<ApisixUpstreamPortLevelSettingsLoadbalancer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<ApisixUpstreamPortLevelSettingsScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<ApisixUpstreamPortLevelSettingsTimeout>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsHealthCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<ApisixUpstreamPortLevelSettingsHealthCheckActive>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passive: Option<ApisixUpstreamPortLevelSettingsHealthCheckPassive>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsHealthCheckActive {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<ApisixUpstreamPortLevelSettingsHealthCheckActiveHealthy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpPath")]
    pub http_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaders")]
    pub request_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "strictTLS")]
    pub strict_tls: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ApisixUpstreamPortLevelSettingsHealthCheckActiveType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unhealthy: Option<ApisixUpstreamPortLevelSettingsHealthCheckActiveUnhealthy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsHealthCheckActiveHealthy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpCodes")]
    pub http_codes: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successes: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamPortLevelSettingsHealthCheckActiveType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "tcp")]
    Tcp,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsHealthCheckActiveUnhealthy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpCodes")]
    pub http_codes: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpFailures")]
    pub http_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpFailures")]
    pub tcp_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsHealthCheckPassive {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<ApisixUpstreamPortLevelSettingsHealthCheckPassiveHealthy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<ApisixUpstreamPortLevelSettingsHealthCheckPassiveType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unhealthy: Option<ApisixUpstreamPortLevelSettingsHealthCheckPassiveUnhealthy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsHealthCheckPassiveHealthy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpCodes")]
    pub http_codes: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successes: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamPortLevelSettingsHealthCheckPassiveType {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "tcp")]
    Tcp,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsHealthCheckPassiveUnhealthy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpCodes")]
    pub http_codes: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpFailures")]
    pub http_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpFailures")]
    pub tcp_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsLoadbalancer {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hashOn")]
    pub hash_on: Option<ApisixUpstreamPortLevelSettingsLoadbalancerHashOn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "type")]
    pub r#type: ApisixUpstreamPortLevelSettingsLoadbalancerType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamPortLevelSettingsLoadbalancerHashOn {
    #[serde(rename = "vars")]
    Vars,
    #[serde(rename = "vars_combinations")]
    VarsCombinations,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "cookie")]
    Cookie,
    #[serde(rename = "consumer")]
    Consumer,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamPortLevelSettingsLoadbalancerType {
    #[serde(rename = "roundrobin")]
    Roundrobin,
    #[serde(rename = "chash")]
    Chash,
    #[serde(rename = "ewma")]
    Ewma,
    #[serde(rename = "least_conn")]
    LeastConn,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamPortLevelSettingsScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "grpc")]
    Grpc,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamPortLevelSettingsTimeout {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ApisixUpstreamScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "grpc")]
    Grpc,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "grpcs")]
    Grpcs,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamSubsets {
    pub labels: BTreeMap<String, serde_json::Value>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamTimeout {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send: Option<String>,
}

/// ApisixSecret describes the Kubernetes Secret name and namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamTlsSecret {
    pub name: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ApisixUpstreamStatusConditions>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ApisixUpstreamStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

