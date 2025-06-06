// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/redhat-cop/vault-config-operator/redhatcop.redhat.io/v1alpha1/jwtoidcauthengineroles.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// JWTOIDCAuthEngineRoleSpec defines the desired state of JWTOIDCAuthEngineRole
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "redhatcop.redhat.io", version = "v1alpha1", kind = "JWTOIDCAuthEngineRole", plural = "jwtoidcauthengineroles")]
#[kube(namespaced)]
#[kube(status = "JWTOIDCAuthEngineRoleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct JWTOIDCAuthEngineRoleSpec {
    /// If set, a list of OIDC scopes to be used with an OIDC role
    /// The standard scope "openid" is automatically included and need not be specified
    /// kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "OIDCScopes")]
    pub oidc_scopes: Option<Vec<String>>,
    /// The list of allowed values for redirect_uri during OIDC logins
    /// kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRedirectURIs")]
    pub allowed_redirect_ur_is: Option<Vec<String>>,
    /// Authentication is the kube auth configuraiton to be used to execute this request
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication: Option<JWTOIDCAuthEngineRoleAuthentication>,
    /// List of aud claims to match against. Any match is sufficient. Required for "jwt" roles, optional for "oidc" roles
    /// kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundAudiences")]
    pub bound_audiences: Option<Vec<String>>,
    /// If set, a map of claims (keys) to match against respective claim values (values)
    /// The expected value may be a single string or a list of strings
    /// The interpretation of the bound claim values is configured with bound_claims_type
    /// Keys support JSON pointer syntax for referencing claims
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundClaims")]
    pub bound_claims: Option<serde_json::Value>,
    /// Configures the interpretation of the bound_claims values.
    /// If "string" (the default), the values will treated as string literals and must match exactly.
    /// If set to "glob", the values will be interpreted as globs, with * matching any number of characters
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundClaimsType")]
    pub bound_claims_type: Option<String>,
    /// If set, requires that the sub claim matches this value.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "boundSubject")]
    pub bound_subject: Option<String>,
    /// If set, a map of claims (keys) to be copied to specified metadata fields (values)
    /// Keys support JSON pointer syntax for referencing claims
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "claimMappings")]
    pub claim_mappings: Option<BTreeMap<String, String>>,
    /// The amount of leeway to add to all claims to account for clock skew, in seconds.
    /// Defaults to 60 seconds if set to 0 and can be disabled if set to -1.
    /// Accepts an integer number of seconds, or a Go duration format string. Only applicable with "jwt" roles
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clockSkewLeeway")]
    pub clock_skew_leeway: Option<i64>,
    /// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<JWTOIDCAuthEngineRoleConnection>,
    /// The amount of leeway to add to expiration (exp) claims to account for clock skew, in seconds.
    /// Defaults to 150 seconds if set to 0 and can be disabled if set to -1.
    /// Accepts an integer number of seconds, or a Go duration format string. Only applicable with "jwt" roles.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "expirationLeeway")]
    pub expiration_leeway: Option<i64>,
    /// The claim to use to uniquely identify the set of groups to which the user belongs; this will be used as the names for the Identity group aliases created due to a successful login.
    /// The claim value must be a list of strings. Supports JSON pointer syntax for referencing claims
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupsClaim")]
    pub groups_claim: Option<String>,
    /// Specifies the allowable elapsed time in seconds since the last time the user was actively authenticated with the OIDC provider
    /// If set, the max_age request parameter will be included in the authentication request
    /// See AuthRequest for additional details
    /// Accepts an integer number of seconds, or a Go duration format string
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maxage: Option<i64>,
    /// Name of the role
    pub name: String,
    /// he amount of leeway to add to not before (nbf) claims to account for clock skew, in seconds
    /// Defaults to 150 seconds if set to 0 and can be disabled if set to -1.
    /// Accepts an integer number of seconds, or a Go duration format string. Only applicable with "jwt" roles
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notBeforeLeeway")]
    pub not_before_leeway: Option<i64>,
    /// Path at which to make the configuration.
    /// The final path in Vault will be {[spec.authentication.namespace]}/auth/{spec.path}/groups/{metadata.name}.
    /// The authentication role must have the following capabilities = [ "create", "read", "update", "delete"] on that path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Type of role, either "oidc" (default) or "jwt"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleType")]
    pub role_type: Option<String>,
    /// List of CIDR blocks; if set, specifies blocks of IP addresses which can authenticate successfully, and ties the resulting token to these blocks as well.
    /// kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenBoundCIDRs")]
    pub token_bound_cid_rs: Option<Vec<String>>,
    /// If set, will encode an explicit max TTL onto the token.
    /// This is a hard cap even if token_ttl and token_max_ttl would otherwise allow a renewal.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenExplicitMaxTTL")]
    pub token_explicit_max_ttl: Option<String>,
    /// The maximum lifetime for generated tokens.
    /// This current value of this will be referenced at renewal time
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenMaxTTL")]
    pub token_max_ttl: Option<String>,
    /// If set, the default policy will not be set on generated tokens; otherwise it will be added to the policies set in token_policies
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenNoDefaultPolicy")]
    pub token_no_default_policy: Option<bool>,
    /// The maximum number of times a generated token may be used (within its lifetime); 0 means unlimited.
    /// If you require the token to have the ability to create child tokens, you will need to set this value to 0
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenNumUses")]
    pub token_num_uses: Option<i64>,
    /// The period, if any, to set on the token
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenPeriod")]
    pub token_period: Option<i64>,
    /// List of policies to encode onto generated tokens
    /// Depending on the auth method, this list may be supplemented by user/group/other values
    /// kubebuilder:validation:UniqueItems=true
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenPolicies")]
    pub token_policies: Option<Vec<String>>,
    /// The incremental lifetime for generated tokens
    /// This current value of this will be referenced at renewal time
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenTTL")]
    pub token_ttl: Option<String>,
    /// The type of token that should be generated. Can be service, batch, or default to use the mount's tuned default (which unless changed will be service tokens).
    /// For token store roles, there are two additional possibilities: default-service and default-batch which specify the type to return unless the client requests a different type at generation time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenType")]
    pub token_type: Option<String>,
    /// The claim to use to uniquely identify the user; this will be used as the name for the Identity entity alias created due to a successful login.
    /// The claim value must be a string
    #[serde(rename = "userClaim")]
    pub user_claim: String,
    /// Specifies if the user_claim value uses JSON pointer syntax for referencing claims.
    /// By default, the user_claim value will not use JSON pointer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "userClaimJSONPointer")]
    pub user_claim_json_pointer: Option<bool>,
    /// Log received OIDC tokens and claims when debug-level logging is active
    /// Not recommended in production since sensitive information may be present in OIDC responses
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "verboseOIDCLogging")]
    pub verbose_oidc_logging: Option<bool>,
}

/// Authentication is the kube auth configuraiton to be used to execute this request
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JWTOIDCAuthEngineRoleAuthentication {
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
    pub service_account: Option<JWTOIDCAuthEngineRoleAuthenticationServiceAccount>,
}

/// ServiceAccount is the service account used for the kube auth authentication
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JWTOIDCAuthEngineRoleAuthenticationServiceAccount {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Connection represents the information needed to connect to Vault. This operator uses the standard Vault environment variables to connect to Vault. If you need to override those settings and for example connect to a different Vault instance, you can do with this section of the CR.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JWTOIDCAuthEngineRoleConnection {
    /// Address Address of the Vault server expressed as a URL and port, for example: https://127.0.0.1:8200/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// MaxRetries Maximum number of retries when certain error codes are encountered. The default is 2, for three total attempts. Set this to 0 or less to disable retrying. Error codes that are retried are 412 (client consistency requirement not satisfied) and all 5xx except for 501 (not implemented).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxRetries")]
    pub max_retries: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tLSConfig")]
    pub t_ls_config: Option<JWTOIDCAuthEngineRoleConnectionTLsConfig>,
    /// Timeout Timeout variable. The default value is 60s.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeOut")]
    pub time_out: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JWTOIDCAuthEngineRoleConnectionTLsConfig {
    /// Cacert Path to a PEM-encoded CA certificate file on the local disk. This file is used to verify the Vault server's SSL certificate. This environment variable takes precedence over a cert passed via the secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cacert: Option<String>,
    /// SkipVerify Do not verify Vault's presented certificate before communicating with it. Setting this variable is not recommended and voids Vault's security model.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "skipVerify")]
    pub skip_verify: Option<bool>,
    /// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsSecret")]
    pub tls_secret: Option<JWTOIDCAuthEngineRoleConnectionTLsConfigTlsSecret>,
    /// TLSServerName Name to use as the SNI host when connecting via TLS.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tlsServerName")]
    pub tls_server_name: Option<String>,
}

/// TLSSecret namespace-local secret containing the tls material for the connection. the expected keys for the secret are: ca bundle -> "ca.crt", certificate -> "tls.crt", key -> "tls.key"
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JWTOIDCAuthEngineRoleConnectionTLsConfigTlsSecret {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// JWTOIDCAuthEngineRoleStatus defines the observed state of JWTOIDCAuthEngineRole
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JWTOIDCAuthEngineRoleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

