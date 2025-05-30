// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/longhorn/longhorn/longhorn.io/v1beta2/instancemanagers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// InstanceManagerSpec defines the desired state of the Longhorn instance manager
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "longhorn.io", version = "v1beta2", kind = "InstanceManager", plural = "instancemanagers")]
#[kube(namespaced)]
#[kube(status = "InstanceManagerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct InstanceManagerSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEngine")]
    pub data_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEngineSpec")]
    pub data_engine_spec: Option<InstanceManagerDataEngineSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeID")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<InstanceManagerType>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerDataEngineSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v2: Option<InstanceManagerDataEngineSpecV2>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerDataEngineSpecV2 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuMask")]
    pub cpu_mask: Option<String>,
}

/// InstanceManagerSpec defines the desired state of the Longhorn instance manager
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum InstanceManagerType {
    #[serde(rename = "aio")]
    Aio,
    #[serde(rename = "engine")]
    Engine,
    #[serde(rename = "replica")]
    Replica,
}

/// InstanceManagerStatus defines the observed state of the Longhorn instance manager
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiMinVersion")]
    pub api_min_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backingImages")]
    pub backing_images: Option<BTreeMap<String, InstanceManagerStatusBackingImages>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentState")]
    pub current_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEngineStatus")]
    pub data_engine_status: Option<InstanceManagerStatusDataEngineStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceEngines")]
    pub instance_engines: Option<BTreeMap<String, InstanceManagerStatusInstanceEngines>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "instanceReplicas")]
    pub instance_replicas: Option<BTreeMap<String, InstanceManagerStatusInstanceReplicas>>,
    /// Deprecated: Replaced by InstanceEngines and InstanceReplicas
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instances: Option<BTreeMap<String, InstanceManagerStatusInstances>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ownerID")]
    pub owner_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyApiMinVersion")]
    pub proxy_api_min_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "proxyApiVersion")]
    pub proxy_api_version: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusBackingImages {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "currentChecksum")]
    pub current_checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "diskUUID")]
    pub disk_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusDataEngineStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub v2: Option<InstanceManagerStatusDataEngineStatusV2>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusDataEngineStatusV2 {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cpuMask")]
    pub cpu_mask: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstanceEngines {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<InstanceManagerStatusInstanceEnginesSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceManagerStatusInstanceEnginesStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstanceEnginesSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEngine")]
    pub data_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstanceEnginesStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorMsg")]
    pub error_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portEnd")]
    pub port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portStart")]
    pub port_start: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPortEnd")]
    pub target_port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPortStart")]
    pub target_port_start: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ublkID")]
    pub ublk_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstanceReplicas {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<InstanceManagerStatusInstanceReplicasSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceManagerStatusInstanceReplicasStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstanceReplicasSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEngine")]
    pub data_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstanceReplicasStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorMsg")]
    pub error_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portEnd")]
    pub port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portStart")]
    pub port_start: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPortEnd")]
    pub target_port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPortStart")]
    pub target_port_start: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ublkID")]
    pub ublk_id: Option<i32>,
}

/// Deprecated: Replaced by InstanceEngines and InstanceReplicas
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstances {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<InstanceManagerStatusInstancesSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<InstanceManagerStatusInstancesStatus>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstancesSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataEngine")]
    pub data_engine: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct InstanceManagerStatusInstancesStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<BTreeMap<String, bool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorMsg")]
    pub error_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listen: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portEnd")]
    pub port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "portStart")]
    pub port_start: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPortEnd")]
    pub target_port_end: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetPortStart")]
    pub target_port_start: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ublkID")]
    pub ublk_id: Option<i32>,
}

