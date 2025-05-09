// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tinkerbell/tink/tinkerbell.org/v1alpha1/templates.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// TemplateSpec defines the desired state of Template.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "tinkerbell.org", version = "v1alpha1", kind = "Template", plural = "templates")]
#[kube(namespaced)]
#[kube(status = "TemplateStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TemplateSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

/// TemplateStatus defines the observed state of Template.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TemplateStatus {
    /// TemplateState represents the template state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

