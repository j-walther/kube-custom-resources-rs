// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/redhat-cop/vault-config-operator/redhatcop.redhat.io/v1alpha1/quaysecretengineroles.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// QuaySecretEngineRoleSpec defines the desired state of QuaySecretEngineRole
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "QuaySecretEngineRole", plural = "quaysecretengineroles")]
#[kube(namespaced)]
#[kube(status = "QuaySecretEngineRoleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct QuaySecretEngineRoleSpec {
    /// TTL Time-to-Live for the credential
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "TTL")]
    pub ttl: Option<String>,
    /// Authentication is the kube auth configuration to be used to execute this request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<QuaySecretEngineRoleAuthentication>,
    /// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<QuaySecretEngineRoleConnection>,
    /// CreateRepositories Access to create Quay repositories.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "createRepositories")]
    pub create_repositories: Option<bool>,
    /// DefaultPermission Permissions granted to the Robot Account in newly created repositories
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultPermission")]
    pub default_permission: Option<QuaySecretEngineRoleDefaultPermission>,
    /// MaxTTL Maximum Time-to-Live for the credential
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxTTL")]
    pub max_ttl: Option<String>,
    /// The name of the obejct created in Vault. If this is specified it takes precedence over {metatada.name}
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// NamespaceName Name of the Quay account.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceName")]
    pub namespace_name: Option<String>,
    /// NamespaceType Type of account namespace to manage.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceType")]
    pub namespace_type: Option<QuaySecretEngineRoleNamespaceType>,
    /// Path at which to make the configuration.
    /// The final path in Vault will be {[spec.authentication.namespace]}/{spec.path}/roles/{metadata.name}.
    /// The authentication role must have the following capabilities = [ "create", "read", "update", "delete"] on that path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Teams Permissions granted to the Robot Account to Repositories.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<BTreeMap<String, String>>,
    /// Teams Permissions granted to the Robot Account to Teams.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams: Option<BTreeMap<String, String>>,
}

/// Authentication is the kube auth configuration to be used to execute this request
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuaySecretEngineRoleAuthentication {
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
    pub service_account: Option<QuaySecretEngineRoleAuthenticationServiceAccount>,
}

/// ServiceAccount is the service account used for the kube auth authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuaySecretEngineRoleAuthenticationServiceAccount {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuaySecretEngineRoleConnection {
    /// Address Address of the Vault server expressed as a URL and port, for example: https://127.0.0.1:8200/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// MaxRetries Maximum number of retries when certain error codes are encountered. The default is 2, for three total attempts. Set this to 0 or less to disable retrying. Error codes that are retried are 412 (client consistency requirement not satisfied) and all 5xx except for 501 (not implemented).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tLSConfig")]
    pub t_ls_config: Option<QuaySecretEngineRoleConnectionTLsConfig>,
    /// Timeout Timeout variable. The default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeOut")]
    pub time_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuaySecretEngineRoleConnectionTLsConfig {
    /// Cacert Path to a PEM-encoded CA certificate file on the local disk. This file is used to verify the Vault server's SSL certificate. This environment variable takes precedence over a cert passed via the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// SkipVerify Do not verify Vault's presented certificate before communicating with it. Setting this variable is not recommended and voids Vault's security model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipVerify")]
    pub skip_verify: Option<bool>,
    /// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<QuaySecretEngineRoleConnectionTLsConfigTlsSecret>,
    /// TLSServerName Name to use as the SNI host when connecting via TLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsServerName")]
    pub tls_server_name: Option<String>,
}

/// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuaySecretEngineRoleConnectionTLsConfigTlsSecret {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// QuaySecretEngineRoleSpec defines the desired state of QuaySecretEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuaySecretEngineRoleDefaultPermission {
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

/// QuaySecretEngineRoleSpec defines the desired state of QuaySecretEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum QuaySecretEngineRoleNamespaceType {
    #[serde(rename = "organization")]
    Organization,
    #[serde(rename = "user")]
    User,
}

/// QuaySecretEngineRoleStatus defines the observed state of QuaySecretEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct QuaySecretEngineRoleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

