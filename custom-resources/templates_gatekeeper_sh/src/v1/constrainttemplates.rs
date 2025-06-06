// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/open-policy-agent/gatekeeper/templates.gatekeeper.sh/v1/constrainttemplates.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ConstraintTemplateSpec defines the desired state of ConstraintTemplate.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "templates.gatekeeper.sh", version = "v1", kind = "ConstraintTemplate", plural = "constrainttemplates")]
#[kube(status = "ConstraintTemplateStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ConstraintTemplateSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub crd: Option<ConstraintTemplateCrd>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<ConstraintTemplateTargets>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateCrd {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ConstraintTemplateCrdSpec>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateCrdSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub names: Option<ConstraintTemplateCrdSpecNames>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<ConstraintTemplateCrdSpecValidation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateCrdSpecNames {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "shortNames")]
    pub short_names: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateCrdSpecValidation {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "legacySchema")]
    pub legacy_schema: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openAPIV3Schema")]
    pub open_apiv3_schema: Option<BTreeMap<String, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateTargets {
    /// The source code options for the constraint template. "Rego" can only
    /// be specified in one place (either here or in the "rego" field)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<Vec<ConstraintTemplateTargetsCode>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub libs: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rego: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateTargetsCode {
    /// The engine used to evaluate the code. Example: "Rego". Required.
    pub engine: String,
    /// The source code for the template. Required.
    pub source: serde_json::Value,
}

/// ConstraintTemplateStatus defines the observed state of ConstraintTemplate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "byPod")]
    pub by_pod: Option<Vec<ConstraintTemplateStatusByPod>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
}

/// ByPodStatus defines the observed state of ConstraintTemplate as seen by
/// an individual controller
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateStatusByPod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ConstraintTemplateStatusByPodErrors>>,
    /// a unique identifier for the pod that wrote the status
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
}

/// CreateCRDError represents a single error caught during parsing, compiling, etc.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ConstraintTemplateStatusByPodErrors {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    pub message: String,
}

