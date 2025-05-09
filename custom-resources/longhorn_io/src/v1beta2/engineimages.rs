// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/engineimages.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// EngineImageSpec defines the desired state of the Longhorn engine image
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "EngineImage", plural = "engineimages")]
#[kube(namespaced)]
#[kube(status = "EngineImageStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct EngineImageSpec {
    pub image: String,
}

/// EngineImageStatus defines the observed state of the Longhorn engine image
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EngineImageStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "buildDate")]
    pub build_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cliAPIMinVersion")]
    pub cli_api_min_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cliAPIVersion")]
    pub cli_api_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controllerAPIMinVersion")]
    pub controller_api_min_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controllerAPIVersion")]
    pub controller_api_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataFormatMinVersion")]
    pub data_format_min_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataFormatVersion")]
    pub data_format_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitCommit")]
    pub git_commit: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incompatible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noRefSince")]
    pub no_ref_since: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeDeploymentMap")]
    pub node_deployment_map: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refCount")]
    pub ref_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

