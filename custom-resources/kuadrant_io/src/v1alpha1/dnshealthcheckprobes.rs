// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/Kuadrant/dns-operator/kuadrant.io/v1alpha1/dnshealthcheckprobes.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// DNSHealthCheckProbeSpec defines the desired state of DNSHealthCheckProbe
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuadrant.io", version = "v1alpha1", kind = "DNSHealthCheckProbe", plural = "dnshealthcheckprobes")]
#[kube(namespaced)]
#[kube(status = "DNSHealthCheckProbeStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DNSHealthCheckProbeSpec {
    /// AdditionalHeadersRef refers to a secret that contains extra headers to send in the probe request, this is primarily useful if an authentication
    /// token is required by the endpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalHeadersRef")]
    pub additional_headers_ref: Option<DNSHealthCheckProbeAdditionalHeadersRef>,
    /// Address to connect to the host on (IP Address (A Record) or hostname (CNAME)).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// AllowInsecureCertificate will instruct the health check probe to not fail on a self-signed or otherwise invalid SSL certificate
    /// this is primarily used in development or testing environments and is set by the --insecure-health-checks flag
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowInsecureCertificate")]
    pub allow_insecure_certificate: Option<bool>,
    /// FailureThreshold is a limit of consecutive failures that must occur for a host to be considered unhealthy
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureThreshold")]
    pub failure_threshold: Option<i64>,
    /// Hostname is the value sent in the host header, to route the request to the correct service
    /// Represents a root host of the parent DNS Record.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// Interval defines how frequently this probe should execute
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Path is the path to append to the host to reach the expected health check.
    /// Must start with "?" or "/", contain only valid URL characters and end with alphanumeric char or "/". For example "/" or "/healthz" are common
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Port to connect to the host on. Must be either 80, 443 or 1024-49151
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// Protocol to use when connecting to the host, valid values are "HTTP" or "HTTPS"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// AdditionalHeadersRef refers to a secret that contains extra headers to send in the probe request, this is primarily useful if an authentication
/// token is required by the endpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSHealthCheckProbeAdditionalHeadersRef {
    pub name: String,
}

/// DNSHealthCheckProbeStatus defines the observed state of DNSHealthCheckProbe
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DNSHealthCheckProbeStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consecutiveFailures")]
    pub consecutive_failures: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthy: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

