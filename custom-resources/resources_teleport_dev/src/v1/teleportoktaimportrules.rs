// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/gravitational/teleport/resources.teleport.dev/v1/teleportoktaimportrules.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// OktaImportRule resource definition v1 from Teleport
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "resources.teleport.dev", version = "v1", kind = "TeleportOktaImportRule", plural = "teleportoktaimportrules")]
#[kube(namespaced)]
#[kube(status = "TeleportOktaImportRuleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TeleportOktaImportRuleSpec {
    /// Mappings is a list of matches that will map match conditions to labels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<TeleportOktaImportRuleMappings>>,
    /// Priority represents the priority of the rule application. Lower numbered rules will be applied first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportOktaImportRuleMappings {
    /// AddLabels specifies which labels to add if any of the previous matches match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_labels: Option<TeleportOktaImportRuleMappingsAddLabels>,
    /// Match is a set of matching rules for this mapping. If any of these match, then the mapping will be applied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "match")]
    pub r#match: Option<Vec<TeleportOktaImportRuleMappingsMatch>>,
}

/// AddLabels specifies which labels to add if any of the previous matches match.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportOktaImportRuleMappingsAddLabels {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportOktaImportRuleMappingsMatch {
    /// AppIDs is a list of app IDs to match against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// AppNameRegexes is a list of regexes to match against app names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_name_regexes: Option<Vec<String>>,
    /// GroupIDs is a list of group IDs to match against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// GroupNameRegexes is a list of regexes to match against group names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_name_regexes: Option<Vec<String>>,
}

/// Status defines the observed state of the Teleport resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TeleportOktaImportRuleStatus {
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "teleportResourceID")]
    pub teleport_resource_id: Option<i64>,
}

