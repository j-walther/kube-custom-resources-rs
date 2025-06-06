// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/CleverCloud/clever-operator/api.clever-cloud.com/v1alpha1/kvs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "api.clever-cloud.com", version = "v1alpha1", kind = "KV", plural = "kvs")]
#[kube(namespaced)]
#[kube(status = "KVStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KVSpec {
    pub instance: KVInstance,
    pub organisation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KVInstance {
    pub region: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KVStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addon: Option<String>,
}

