// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/istio/istio/networking.istio.io/v1alpha3/sidecars.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Configuration affecting network reachability of a sidecar. See more details at: https://istio.io/docs/reference/config/networking/sidecar.html
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "networking.istio.io", version = "v1alpha3", kind = "Sidecar", plural = "sidecars")]
#[kube(namespaced)]
#[kube(status = "SidecarStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SidecarSpec {
    /// Egress specifies the configuration of the sidecar for processing outbound traffic from the attached workload instance to other services in the mesh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub egress: Option<Vec<SidecarEgress>>,
    /// Settings controlling the volume of connections Envoy will accept from the network.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inboundConnectionPool")]
    pub inbound_connection_pool: Option<SidecarInboundConnectionPool>,
    /// Ingress specifies the configuration of the sidecar for processing inbound traffic to the attached workload instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<SidecarIngress>>,
    /// Set the default behavior of the sidecar for handling outbound traffic from the application.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "outboundTrafficPolicy")]
    pub outbound_traffic_policy: Option<SidecarOutboundTrafficPolicy>,
    /// Criteria used to select the specific set of pods/VMs on which this `Sidecar` configuration should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workloadSelector")]
    pub workload_selector: Option<SidecarWorkloadSelector>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarEgress {
    /// The IP(IPv4 or IPv6) or the Unix domain socket to which the listener should be bound to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    /// When the bind address is an IP, the captureMode option dictates how traffic to the listener is expected to be captured (or not).
    /// 
    /// Valid Options: DEFAULT, IPTABLES, NONE
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "captureMode")]
    pub capture_mode: Option<SidecarEgressCaptureMode>,
    /// One or more service hosts exposed by the listener in `namespace/dnsName` format.
    pub hosts: Vec<String>,
    /// The port associated with the listener.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarEgressPort>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarEgressCaptureMode {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "IPTABLES")]
    Iptables,
    #[serde(rename = "NONE")]
    None,
}

/// The port associated with the listener.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarEgressPort {
    /// Label assigned to the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A valid non-negative integer port number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// The protocol exposed on the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

/// Settings controlling the volume of connections Envoy will accept from the network.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarInboundConnectionPool {
    /// HTTP connection pool settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<SidecarInboundConnectionPoolHttp>,
    /// Settings common to both HTTP and TCP upstream connections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<SidecarInboundConnectionPoolTcp>,
}

/// HTTP connection pool settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarInboundConnectionPoolHttp {
    /// Specify if http1.1 connection should be upgraded to http2 for the associated destination.
    /// 
    /// Valid Options: DEFAULT, DO_NOT_UPGRADE, UPGRADE
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "h2UpgradePolicy")]
    pub h2_upgrade_policy: Option<SidecarInboundConnectionPoolHttpH2UpgradePolicy>,
    /// Maximum number of requests that will be queued while waiting for a ready connection pool connection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "http1MaxPendingRequests")]
    pub http1_max_pending_requests: Option<i32>,
    /// Maximum number of active requests to a destination.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "http2MaxRequests")]
    pub http2_max_requests: Option<i32>,
    /// The idle timeout for upstream connection pool connections.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
    /// The maximum number of concurrent streams allowed for a peer on one HTTP/2 connection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConcurrentStreams")]
    pub max_concurrent_streams: Option<i32>,
    /// Maximum number of requests per connection to a backend.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRequestsPerConnection")]
    pub max_requests_per_connection: Option<i32>,
    /// Maximum number of retries that can be outstanding to all hosts in a cluster at a given time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i32>,
    /// If set to true, client protocol will be preserved while initiating connection to backend.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useClientProtocol")]
    pub use_client_protocol: Option<bool>,
}

/// HTTP connection pool settings.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarInboundConnectionPoolHttpH2UpgradePolicy {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "DO_NOT_UPGRADE")]
    DoNotUpgrade,
    #[serde(rename = "UPGRADE")]
    Upgrade,
}

/// Settings common to both HTTP and TCP upstream connections.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarInboundConnectionPoolTcp {
    /// TCP connection timeout.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectTimeout")]
    pub connect_timeout: Option<String>,
    /// The idle timeout for TCP connections.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
    /// The maximum duration of a connection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnectionDuration")]
    pub max_connection_duration: Option<String>,
    /// Maximum number of HTTP1 /TCP connections to a destination host.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnections")]
    pub max_connections: Option<i32>,
    /// If set then set SO_KEEPALIVE on the socket to enable TCP Keepalives.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpKeepalive")]
    pub tcp_keepalive: Option<SidecarInboundConnectionPoolTcpTcpKeepalive>,
}

/// If set then set SO_KEEPALIVE on the socket to enable TCP Keepalives.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarInboundConnectionPoolTcpTcpKeepalive {
    /// The time duration between keep-alive probes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Maximum number of keepalive probes to send without response before deciding the connection is dead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probes: Option<i64>,
    /// The time duration a connection needs to be idle before keep-alive probes start being sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarIngress {
    /// The IP(IPv4 or IPv6) to which the listener should be bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bind: Option<String>,
    /// The captureMode option dictates how traffic to the listener is expected to be captured (or not).
    /// 
    /// Valid Options: DEFAULT, IPTABLES, NONE
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "captureMode")]
    pub capture_mode: Option<SidecarIngressCaptureMode>,
    /// Settings controlling the volume of connections Envoy will accept from the network.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectionPool")]
    pub connection_pool: Option<SidecarIngressConnectionPool>,
    /// The IP endpoint or Unix domain socket to which traffic should be forwarded to.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultEndpoint")]
    pub default_endpoint: Option<String>,
    /// The port associated with the listener.
    pub port: SidecarIngressPort,
    /// Set of TLS related options that will enable TLS termination on the sidecar for requests originating from outside the mesh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<SidecarIngressTls>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarIngressCaptureMode {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "IPTABLES")]
    Iptables,
    #[serde(rename = "NONE")]
    None,
}

/// Settings controlling the volume of connections Envoy will accept from the network.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarIngressConnectionPool {
    /// HTTP connection pool settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<SidecarIngressConnectionPoolHttp>,
    /// Settings common to both HTTP and TCP upstream connections.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcp: Option<SidecarIngressConnectionPoolTcp>,
}

/// HTTP connection pool settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarIngressConnectionPoolHttp {
    /// Specify if http1.1 connection should be upgraded to http2 for the associated destination.
    /// 
    /// Valid Options: DEFAULT, DO_NOT_UPGRADE, UPGRADE
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "h2UpgradePolicy")]
    pub h2_upgrade_policy: Option<SidecarIngressConnectionPoolHttpH2UpgradePolicy>,
    /// Maximum number of requests that will be queued while waiting for a ready connection pool connection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "http1MaxPendingRequests")]
    pub http1_max_pending_requests: Option<i32>,
    /// Maximum number of active requests to a destination.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "http2MaxRequests")]
    pub http2_max_requests: Option<i32>,
    /// The idle timeout for upstream connection pool connections.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
    /// The maximum number of concurrent streams allowed for a peer on one HTTP/2 connection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConcurrentStreams")]
    pub max_concurrent_streams: Option<i32>,
    /// Maximum number of requests per connection to a backend.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRequestsPerConnection")]
    pub max_requests_per_connection: Option<i32>,
    /// Maximum number of retries that can be outstanding to all hosts in a cluster at a given time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i32>,
    /// If set to true, client protocol will be preserved while initiating connection to backend.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useClientProtocol")]
    pub use_client_protocol: Option<bool>,
}

/// HTTP connection pool settings.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarIngressConnectionPoolHttpH2UpgradePolicy {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "DO_NOT_UPGRADE")]
    DoNotUpgrade,
    #[serde(rename = "UPGRADE")]
    Upgrade,
}

/// Settings common to both HTTP and TCP upstream connections.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarIngressConnectionPoolTcp {
    /// TCP connection timeout.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "connectTimeout")]
    pub connect_timeout: Option<String>,
    /// The idle timeout for TCP connections.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<String>,
    /// The maximum duration of a connection.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnectionDuration")]
    pub max_connection_duration: Option<String>,
    /// Maximum number of HTTP1 /TCP connections to a destination host.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxConnections")]
    pub max_connections: Option<i32>,
    /// If set then set SO_KEEPALIVE on the socket to enable TCP Keepalives.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tcpKeepalive")]
    pub tcp_keepalive: Option<SidecarIngressConnectionPoolTcpTcpKeepalive>,
}

/// If set then set SO_KEEPALIVE on the socket to enable TCP Keepalives.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarIngressConnectionPoolTcpTcpKeepalive {
    /// The time duration between keep-alive probes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Maximum number of keepalive probes to send without response before deciding the connection is dead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub probes: Option<i64>,
    /// The time duration a connection needs to be idle before keep-alive probes start being sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

/// The port associated with the listener.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarIngressPort {
    /// Label assigned to the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A valid non-negative integer port number.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    /// The protocol exposed on the port.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPort")]
    pub target_port: Option<i64>,
}

/// Set of TLS related options that will enable TLS termination on the sidecar for requests originating from outside the mesh.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarIngressTls {
    /// REQUIRED if mode is `MUTUAL` or `OPTIONAL_MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificates")]
    pub ca_certificates: Option<String>,
    /// OPTIONAL: The path to the file containing the certificate revocation list (CRL) to use in verifying a presented client side certificate.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCrl")]
    pub ca_crl: Option<String>,
    /// Optional: If specified, only support the specified cipher list.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cipherSuites")]
    pub cipher_suites: Option<Vec<String>>,
    /// For gateways running on Kubernetes, the name of the secret that holds the TLS certs including the CA certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialName")]
    pub credential_name: Option<String>,
    /// Same as CredentialName but for multiple certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialNames")]
    pub credential_names: Option<Vec<String>>,
    /// If set to true, the load balancer will send a 301 redirect for all http connections, asking the clients to use HTTPS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpsRedirect")]
    pub https_redirect: Option<bool>,
    /// Optional: Maximum TLS protocol version.
    /// 
    /// Valid Options: TLS_AUTO, TLSV1_0, TLSV1_1, TLSV1_2, TLSV1_3
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxProtocolVersion")]
    pub max_protocol_version: Option<SidecarIngressTlsMaxProtocolVersion>,
    /// Optional: Minimum TLS protocol version.
    /// 
    /// Valid Options: TLS_AUTO, TLSV1_0, TLSV1_1, TLSV1_2, TLSV1_3
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minProtocolVersion")]
    pub min_protocol_version: Option<SidecarIngressTlsMinProtocolVersion>,
    /// Optional: Indicates whether connections to this port should be secured using TLS.
    /// 
    /// Valid Options: PASSTHROUGH, SIMPLE, MUTUAL, AUTO_PASSTHROUGH, ISTIO_MUTUAL, OPTIONAL_MUTUAL
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<SidecarIngressTlsMode>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKey")]
    pub private_key: Option<String>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCertificate")]
    pub server_certificate: Option<String>,
    /// A list of alternate names to verify the subject identity in the certificate presented by the client.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<String>>,
    /// Only one of `server_certificate`, `private_key`, `ca_certificates` or `credential_name` or `credential_names` or `tls_certificates` should be specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsCertificates")]
    pub tls_certificates: Option<Vec<SidecarIngressTlsTlsCertificates>>,
    /// An optional list of hex-encoded SHA-256 hashes of the authorized client certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateHash")]
    pub verify_certificate_hash: Option<Vec<String>>,
    /// An optional list of base64-encoded SHA-256 hashes of the SPKIs of authorized client certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verifyCertificateSpki")]
    pub verify_certificate_spki: Option<Vec<String>>,
}

/// Set of TLS related options that will enable TLS termination on the sidecar for requests originating from outside the mesh.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarIngressTlsMaxProtocolVersion {
    #[serde(rename = "TLS_AUTO")]
    TlsAuto,
    #[serde(rename = "TLSV1_0")]
    Tlsv10,
    #[serde(rename = "TLSV1_1")]
    Tlsv11,
    #[serde(rename = "TLSV1_2")]
    Tlsv12,
    #[serde(rename = "TLSV1_3")]
    Tlsv13,
}

/// Set of TLS related options that will enable TLS termination on the sidecar for requests originating from outside the mesh.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarIngressTlsMinProtocolVersion {
    #[serde(rename = "TLS_AUTO")]
    TlsAuto,
    #[serde(rename = "TLSV1_0")]
    Tlsv10,
    #[serde(rename = "TLSV1_1")]
    Tlsv11,
    #[serde(rename = "TLSV1_2")]
    Tlsv12,
    #[serde(rename = "TLSV1_3")]
    Tlsv13,
}

/// Set of TLS related options that will enable TLS termination on the sidecar for requests originating from outside the mesh.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarIngressTlsMode {
    #[serde(rename = "PASSTHROUGH")]
    Passthrough,
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "MUTUAL")]
    Mutual,
    #[serde(rename = "AUTO_PASSTHROUGH")]
    AutoPassthrough,
    #[serde(rename = "ISTIO_MUTUAL")]
    IstioMutual,
    #[serde(rename = "OPTIONAL_MUTUAL")]
    OptionalMutual,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarIngressTlsTlsCertificates {
    /// REQUIRED if mode is `MUTUAL` or `OPTIONAL_MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificates")]
    pub ca_certificates: Option<String>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateKey")]
    pub private_key: Option<String>,
    /// REQUIRED if mode is `SIMPLE` or `MUTUAL`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serverCertificate")]
    pub server_certificate: Option<String>,
}

/// Set the default behavior of the sidecar for handling outbound traffic from the application.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarOutboundTrafficPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "egressProxy")]
    pub egress_proxy: Option<SidecarOutboundTrafficPolicyEgressProxy>,
    /// 
    /// 
    /// Valid Options: REGISTRY_ONLY, ALLOW_ANY
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<SidecarOutboundTrafficPolicyMode>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarOutboundTrafficPolicyEgressProxy {
    /// The name of a service from the service registry.
    pub host: String,
    /// Specifies the port on the host that is being addressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<SidecarOutboundTrafficPolicyEgressProxyPort>,
    /// The name of a subset within the service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subset: Option<String>,
}

/// Specifies the port on the host that is being addressed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarOutboundTrafficPolicyEgressProxyPort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

/// Set the default behavior of the sidecar for handling outbound traffic from the application.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarOutboundTrafficPolicyMode {
    #[serde(rename = "REGISTRY_ONLY")]
    RegistryOnly,
    #[serde(rename = "ALLOW_ANY")]
    AllowAny,
}

/// Criteria used to select the specific set of pods/VMs on which this `Sidecar` configuration should be applied.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarWorkloadSelector {
    /// One or more labels that indicate a specific set of pods/VMs on which the configuration should be applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarStatus {
    /// Current service state of the resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<IntOrString>,
    /// Includes any errors or warnings detected by Istio's analyzers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationMessages")]
    pub validation_messages: Option<Vec<SidecarStatusValidationMessages>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarStatusValidationMessages {
    /// A url pointing to the Istio documentation for this specific error type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "documentationUrl")]
    pub documentation_url: Option<String>,
    /// Represents how severe a message is.
    /// 
    /// Valid Options: UNKNOWN, ERROR, WARNING, INFO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<SidecarStatusValidationMessagesLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<SidecarStatusValidationMessagesType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SidecarStatusValidationMessagesLevel {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ERROR")]
    Error,
    #[serde(rename = "WARNING")]
    Warning,
    #[serde(rename = "INFO")]
    Info,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SidecarStatusValidationMessagesType {
    /// A 7 character code matching `^IST[0-9]{4}$` intended to uniquely identify the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable name for the message type.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

