// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1beta1/localqueues.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// LocalQueueSpec defines the desired state of LocalQueue
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kueue.x-k8s.io", version = "v1beta1", kind = "LocalQueue", plural = "localqueues")]
#[kube(namespaced)]
#[kube(status = "LocalQueueStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct LocalQueueSpec {
    /// clusterQueue is a reference to a clusterQueue that backs this localQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterQueue")]
    pub cluster_queue: Option<String>,
    /// stopPolicy - if set to a value different from None, the LocalQueue is considered Inactive,
    /// no new reservation being made.
    /// 
    /// Depending on its value, its associated workloads will:
    /// 
    /// - None - Workloads are admitted
    /// - HoldAndDrain - Admitted workloads are evicted and Reserving workloads will cancel the reservation.
    /// - Hold - Admitted workloads will run to completion and Reserving workloads will cancel the reservation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stopPolicy")]
    pub stop_policy: Option<LocalQueueStopPolicy>,
}

/// LocalQueueSpec defines the desired state of LocalQueue
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum LocalQueueStopPolicy {
    None,
    Hold,
    HoldAndDrain,
}

/// LocalQueueStatus defines the observed state of LocalQueue
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalQueueStatus {
    /// admittedWorkloads is the number of workloads in this LocalQueue
    /// admitted to a ClusterQueue and that haven't finished yet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "admittedWorkloads")]
    pub admitted_workloads: Option<i32>,
    /// Conditions hold the latest available observations of the LocalQueue
    /// current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// flavorsUsage are the used quotas, by flavor currently in use by the
    /// workloads assigned to this LocalQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorUsage")]
    pub flavor_usage: Option<Vec<LocalQueueStatusFlavorUsage>>,
    /// flavors lists all currently available ResourceFlavors in specified ClusterQueue.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flavors: Option<Vec<LocalQueueStatusFlavors>>,
    /// flavorsReservation are the reserved quotas, by flavor currently in use by the
    /// workloads assigned to this LocalQueue.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flavorsReservation")]
    pub flavors_reservation: Option<Vec<LocalQueueStatusFlavorsReservation>>,
    /// PendingWorkloads is the number of Workloads in the LocalQueue not yet admitted to a ClusterQueue
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingWorkloads")]
    pub pending_workloads: Option<i32>,
    /// reservingWorkloads is the number of workloads in this LocalQueue
    /// reserving quota in a ClusterQueue and that haven't finished yet.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reservingWorkloads")]
    pub reserving_workloads: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalQueueStatusFlavorUsage {
    /// name of the flavor.
    pub name: String,
    /// resources lists the quota usage for the resources in this flavor.
    pub resources: Vec<LocalQueueStatusFlavorUsageResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalQueueStatusFlavorUsageResources {
    /// name of the resource.
    pub name: String,
    /// total is the total quantity of used quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<IntOrString>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalQueueStatusFlavors {
    /// name of the flavor.
    pub name: String,
    /// nodeLabels are labels that associate the ResourceFlavor with Nodes that
    /// have the same labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeLabels")]
    pub node_labels: Option<BTreeMap<String, String>>,
    /// nodeTaints are taints that the nodes associated with this ResourceFlavor
    /// have.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeTaints")]
    pub node_taints: Option<Vec<LocalQueueStatusFlavorsNodeTaints>>,
    /// resources used in the flavor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    /// topology is the topology that associated with this ResourceFlavor.
    /// 
    /// This is an alpha field and requires enabling the TopologyAwareScheduling
    /// feature gate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topology: Option<LocalQueueStatusFlavorsTopology>,
}

/// The node this Taint is attached to has the "effect" on
/// any pod that does not tolerate the Taint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalQueueStatusFlavorsNodeTaints {
    /// Required. The effect of the taint on pods
    /// that do not tolerate the taint.
    /// Valid effects are NoSchedule, PreferNoSchedule and NoExecute.
    pub effect: String,
    /// Required. The taint key to be applied to a node.
    pub key: String,
    /// TimeAdded represents the time at which the taint was added.
    /// It is only written for NoExecute taints.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timeAdded")]
    pub time_added: Option<String>,
    /// The taint value corresponding to the taint key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// topology is the topology that associated with this ResourceFlavor.
/// 
/// This is an alpha field and requires enabling the TopologyAwareScheduling
/// feature gate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalQueueStatusFlavorsTopology {
    /// levels define the levels of topology.
    pub levels: Vec<String>,
    /// name is the name of the topology.
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalQueueStatusFlavorsReservation {
    /// name of the flavor.
    pub name: String,
    /// resources lists the quota usage for the resources in this flavor.
    pub resources: Vec<LocalQueueStatusFlavorsReservationResources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LocalQueueStatusFlavorsReservationResources {
    /// name of the resource.
    pub name: String,
    /// total is the total quantity of used quota.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<IntOrString>,
}

