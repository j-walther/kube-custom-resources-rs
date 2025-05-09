// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/gravitational/teleport/resources.teleport.dev/v2/teleportusers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// User resource definition v2 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v2", kind = "TeleportUser", plural = "teleportusers")]
#[kube(namespaced)]
#[kube(status = "TeleportUserStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TeleportUserSpec {
    /// GithubIdentities list associated Github OAuth2 identities that let user log in using externally verified identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github_identities: Option<Vec<TeleportUserGithubIdentities>>,
    /// OIDCIdentities lists associated OpenID Connect identities that let user log in using externally verified identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oidc_identities: Option<Vec<TeleportUserOidcIdentities>>,
    /// Roles is a list of roles assigned to user
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// SAMLIdentities lists associated SAML identities that let user log in using externally verified identity
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saml_identities: Option<Vec<TeleportUserSamlIdentities>>,
    /// Traits are key/value pairs received from an identity provider (through OIDC claims or SAML assertions) or from a system administrator for local accounts. Traits are used to populate role variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub traits: Option<BTreeMap<String, String>>,
    /// TrustedDeviceIDs contains the IDs of trusted devices enrolled by the user.  Note that SSO users are transient and thus may contain an empty TrustedDeviceIDs field, even though the user->device association exists under the Device Trust subsystem. Do not rely on this field to determine device associations or ownership, it exists for legacy/informative purposes only.  Managed by the Device Trust subsystem, avoid manual edits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trusted_device_ids: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportUserGithubIdentities {
    /// ConnectorID is id of registered OIDC connector, e.g. 'google-example.com'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// SAMLSingleLogoutURL is the SAML Single log-out URL to initiate SAML SLO (single log-out), if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "samlSingleLogoutUrl")]
    pub saml_single_logout_url: Option<String>,
    /// UserID is the ID of the identity. Some connectors like GitHub have an unique ID apart from the username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Username is username supplied by external identity provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportUserOidcIdentities {
    /// ConnectorID is id of registered OIDC connector, e.g. 'google-example.com'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// SAMLSingleLogoutURL is the SAML Single log-out URL to initiate SAML SLO (single log-out), if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "samlSingleLogoutUrl")]
    pub saml_single_logout_url: Option<String>,
    /// UserID is the ID of the identity. Some connectors like GitHub have an unique ID apart from the username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Username is username supplied by external identity provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportUserSamlIdentities {
    /// ConnectorID is id of registered OIDC connector, e.g. 'google-example.com'
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// SAMLSingleLogoutURL is the SAML Single log-out URL to initiate SAML SLO (single log-out), if applicable.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "samlSingleLogoutUrl")]
    pub saml_single_logout_url: Option<String>,
    /// UserID is the ID of the identity. Some connectors like GitHub have an unique ID apart from the username.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Username is username supplied by external identity provider
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// Status defines the observed state of the Teleport resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportUserStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

