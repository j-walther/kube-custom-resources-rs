// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/oidcconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// OIDCConfigSpec defines the desired state of OIDCConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "OIDCConfig", plural = "oidcconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct OIDCConfigSpec {
    /// ClientId defines the client ID for the OpenID Connect client
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientId")]
    pub client_id: Option<String>,
    /// GroupsClaim defines the name of a custom OpenID Connect claim for specifying user groups
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupsClaim")]
    pub groups_claim: Option<String>,
    /// GroupsPrefix defines a string to be prefixed to all groups to prevent conflicts with other authentication strategies
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "groupsPrefix")]
    pub groups_prefix: Option<String>,
    /// IssuerUrl defines the URL of the OpenID issuer, only HTTPS scheme will be accepted
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "issuerUrl")]
    pub issuer_url: Option<String>,
    /// RequiredClaims defines a key=value pair that describes a required claim in the ID Token
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredClaims")]
    pub required_claims: Option<Vec<OIDCConfigRequiredClaims>>,
    /// UsernameClaim defines the OpenID claim to use as the user name. Note that claims other than the default ('sub') is not guaranteed to be unique and immutable
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usernameClaim")]
    pub username_claim: Option<String>,
    /// UsernamePrefix defines a string to prefixed to all usernames. If not provided, username claims other than 'email' are prefixed by the issuer URL to avoid clashes. To skip any prefixing, provide the value '-'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "usernamePrefix")]
    pub username_prefix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OIDCConfigRequiredClaims {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// OIDCConfigStatus defines the observed state of OIDCConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OIDCConfigStatus {
}

