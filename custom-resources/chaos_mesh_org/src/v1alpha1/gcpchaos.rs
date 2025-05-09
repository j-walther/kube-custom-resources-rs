// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/chaos-mesh/chaos-mesh/chaos-mesh.org/v1alpha1/gcpchaos.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// GCPChaosSpec is the content of the specification for a GCPChaos
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "chaos-mesh.org", version = "v1alpha1", kind = "GCPChaos", plural = "gcpchaos")]
#[kube(namespaced)]
#[kube(status = "GCPChaosStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct GCPChaosSpec {
    /// Action defines the specific gcp chaos action.
    /// Supported action: node-stop / node-reset / disk-loss
    /// Default action: node-stop
    pub action: GCPChaosAction,
    /// The device name of disks to detach.
    /// Needed in disk-loss.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deviceNames")]
    pub device_names: Option<Vec<String>>,
    /// Duration represents the duration of the chaos action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// Instance defines the name of the instance
    pub instance: String,
    /// Project defines the ID of gcp project.
    pub project: String,
    /// RemoteCluster represents the remote cluster where the chaos will be deployed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "remoteCluster")]
    pub remote_cluster: Option<String>,
    /// SecretName defines the name of kubernetes secret. It is used for GCP credentials.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretName")]
    pub secret_name: Option<String>,
    /// Zone defines the zone of gcp project.
    pub zone: String,
}

/// GCPChaosSpec is the content of the specification for a GCPChaos
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GCPChaosAction {
    #[serde(rename = "node-stop")]
    NodeStop,
    #[serde(rename = "node-reset")]
    NodeReset,
    #[serde(rename = "disk-loss")]
    DiskLoss,
}

/// GCPChaosStatus represents the status of a GCPChaos
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPChaosStatus {
    /// The attached disk info strings.
    /// Needed in disk-loss.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attachedDiskStrings")]
    pub attached_disk_strings: Option<Vec<String>>,
    /// Conditions represents the current global condition of the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<GCPChaosStatusConditions>>,
    /// Experiment records the last experiment state.
    pub experiment: GCPChaosStatusExperiment,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPChaosStatusConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPChaosStatusExperiment {
    /// Records are used to track the running status
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerRecords")]
    pub container_records: Option<Vec<GCPChaosStatusExperimentContainerRecords>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "desiredPhase")]
    pub desired_phase: Option<GCPChaosStatusExperimentDesiredPhase>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPChaosStatusExperimentContainerRecords {
    /// Events are the essential details about the injections and recoveries
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GCPChaosStatusExperimentContainerRecordsEvents>>,
    pub id: String,
    /// InjectedCount is a counter to record the sum of successful injections
    #[serde(rename = "injectedCount")]
    pub injected_count: i64,
    pub phase: String,
    /// RecoveredCount is a counter to record the sum of successful recoveries
    #[serde(rename = "recoveredCount")]
    pub recovered_count: i64,
    #[serde(rename = "selectorKey")]
    pub selector_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct GCPChaosStatusExperimentContainerRecordsEvents {
    /// Message is the detail message, e.g. the reason why we failed to inject the chaos
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Operation represents the operation we are doing, when we crate this event
    pub operation: String,
    /// Timestamp is time when we create this event
    pub timestamp: String,
    /// Type means the stage of this event
    #[serde(rename = "type")]
    pub r#type: String,
}

/// Experiment records the last experiment state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GCPChaosStatusExperimentDesiredPhase {
    Run,
    Stop,
}

