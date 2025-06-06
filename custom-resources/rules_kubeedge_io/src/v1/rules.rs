// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubeedge/kubeedge/rules.kubeedge.io/v1/rules.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "rules.kubeedge.io", version = "v1", kind = "Rule", plural = "rules")]
#[kube(namespaced)]
#[kube(status = "RuleStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct RuleSpec {
    /// source is a string value representing where the messages come from. Its
    /// value is the same with ruleendpoint name. For example, my-rest or my-eventbus.
    /// 
    pub source: String,
    /// sourceResource is a map representing the resource info of source. For rest
    /// rule-endpoint type its value is {"path":"/test"}. For eventbus ruleendpoint type its
    /// value is {"topic":"<user define string>","node_name":"edge-node"}
    /// 
    #[serde(rename = "sourceResource")]
    pub source_resource: BTreeMap<String, String>,
    /// target is a string value representing where the messages go to. its value is
    /// the same with ruleendpoint name. For example, my-eventbus or my-rest or my-servicebus.
    /// 
    pub target: String,
    /// targetResource is a map representing the resource info of target. For rest
    /// rule-endpoint type its value is {"resource":"http://a.com"}. For eventbus ruleendpoint
    /// type its value is {"topic":"/test"}. For servicebus rule-endpoint type its value is
    /// {"path":"/request_path"}.
    /// 
    #[serde(rename = "targetResource")]
    pub target_resource: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct RuleStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failMessages")]
    pub fail_messages: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "successMessages")]
    pub success_messages: Option<i64>,
}

