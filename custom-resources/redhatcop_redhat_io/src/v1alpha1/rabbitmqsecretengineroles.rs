// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/redhat-cop/vault-config-operator/redhatcop.redhat.io/v1alpha1/rabbitmqsecretengineroles.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// RabbitMQSecretEngineRoleSpec defines the desired state of RabbitMQSecretEngineRole
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "RabbitMQSecretEngineRole", plural = "rabbitmqsecretengineroles")]
#[kube(namespaced)]
#[kube(status = "RabbitMQSecretEngineRoleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RabbitMQSecretEngineRoleSpec {
    /// Authentication is the k8s auth configuration to be used to execute this request
    pub authentication: RabbitMQSecretEngineRoleAuthentication,
    /// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<RabbitMQSecretEngineRoleConnection>,
    /// The name of the obejct created in Vault. If this is specified it takes precedence over {metatada.name}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Path at which to make the configuration.
    /// The final path in Vault will be {[spec.authentication.namespace]}/{spec.path}/config/{metadata.name}.
    /// The authentication role must have the following capabilities = [ "create", "read", "update", "delete"] on that path.
    pub path: String,
    /// Comma-separated RabbitMQ permissions tags to associate with the user. This determines the level of
    /// access to the RabbitMQ management UI granted to the user. Omitting this field will
    /// lead to a user than can still connect to the cluster through messaging protocols,
    /// but cannot perform any management actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    /// This option requires RabbitMQ 3.7.0 or later.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vhostTopics")]
    pub vhost_topics: Option<Vec<RabbitMQSecretEngineRoleVhostTopics>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vhosts: Option<Vec<RabbitMQSecretEngineRoleVhosts>>,
}

/// Authentication is the k8s auth configuration to be used to execute this request
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleAuthentication {
    /// Namespace is the Vault namespace to be used in all the operations withing this connection/authentication. Only available in Vault Enterprise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Path is the path of the role used for this kube auth authentication. The operator will try to authenticate at {[namespace/]}auth/{spec.path}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Role the role to be used during authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// ServiceAccount is the service account used for the kube auth authentication
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccount")]
    pub service_account: Option<RabbitMQSecretEngineRoleAuthenticationServiceAccount>,
}

/// ServiceAccount is the service account used for the kube auth authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleAuthenticationServiceAccount {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleConnection {
    /// Address Address of the Vault server expressed as a URL and port, for example: https://127.0.0.1:8200/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// MaxRetries Maximum number of retries when certain error codes are encountered. The default is 2, for three total attempts. Set this to 0 or less to disable retrying. Error codes that are retried are 412 (client consistency requirement not satisfied) and all 5xx except for 501 (not implemented).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tLSConfig")]
    pub t_ls_config: Option<RabbitMQSecretEngineRoleConnectionTLsConfig>,
    /// Timeout Timeout variable. The default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeOut")]
    pub time_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleConnectionTLsConfig {
    /// Cacert Path to a PEM-encoded CA certificate file on the local disk. This file is used to verify the Vault server's SSL certificate. This environment variable takes precedence over a cert passed via the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// SkipVerify Do not verify Vault's presented certificate before communicating with it. Setting this variable is not recommended and voids Vault's security model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipVerify")]
    pub skip_verify: Option<bool>,
    /// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<RabbitMQSecretEngineRoleConnectionTLsConfigTlsSecret>,
    /// TLSServerName Name to use as the SNI host when connecting via TLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsServerName")]
    pub tls_server_name: Option<String>,
}

/// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleConnectionTLsConfigTlsSecret {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleVhostTopics {
    /// List of topics to provide
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<RabbitMQSecretEngineRoleVhostTopicsTopics>>,
    /// Name of an existing vhost.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vhostName")]
    pub vhost_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleVhostTopicsTopics {
    /// Permissions to grant to the user in the specific vhost
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RabbitMQSecretEngineRoleVhostTopicsTopicsPermissions>,
    /// Name of an existing topic.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topicName")]
    pub topic_name: Option<String>,
}

/// Permissions to grant to the user in the specific vhost
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleVhostTopicsTopicsPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configure: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleVhosts {
    /// Permissions to grant to the user in the specific vhost.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<RabbitMQSecretEngineRoleVhostsPermissions>,
    /// Name of an existing vhost.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vhostName")]
    pub vhost_name: Option<String>,
}

/// Permissions to grant to the user in the specific vhost.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleVhostsPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configure: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write: Option<String>,
}

/// RabbitMQSecretEngineRoleStatus defines the observed state of RabbitMQSecretEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RabbitMQSecretEngineRoleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

