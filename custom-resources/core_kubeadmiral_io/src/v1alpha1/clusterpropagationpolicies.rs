// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubewharf/kubeadmiral/core.kubeadmiral.io/v1alpha1/clusterpropagationpolicies.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "core.kubeadmiral.io", version = "v1alpha1", kind = "ClusterPropagationPolicy", plural = "clusterpropagationpolicies")]
#[kube(status = "ClusterPropagationPolicyStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct ClusterPropagationPolicySpec {
    /// Configures behaviors related to auto migration. If absent, auto migration will be disabled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoMigration")]
    pub auto_migration: Option<ClusterPropagationPolicyAutoMigration>,
    /// ClusterAffinity is a list of cluster selector terms, the terms are ORed.
    /// A empty or nil ClusterAffinity selects everything.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAffinity")]
    pub cluster_affinity: Option<Vec<ClusterPropagationPolicyClusterAffinity>>,
    /// ClusterSelector is a label query over clusters to consider for scheduling.
    /// An empty or nil ClusterSelector selects everything.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterSelector")]
    pub cluster_selector: Option<BTreeMap<String, String>>,
    /// DisableFollowerScheduling is a boolean that determines if follower scheduling is disabled.
    /// Resources that depend on other resources (e.g. deployments) are called leaders,
    /// and resources that are depended on (e.g. configmaps and secrets) are called followers.
    /// If a leader enables follower scheduling, its followers will additionally be scheduled
    /// to clusters where the leader is scheduled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableFollowerScheduling")]
    pub disable_follower_scheduling: Option<bool>,
    /// MaxClusters is the maximum number of replicas that the federated object can be propagated to.
    /// The maximum number of clusters is unbounded if no value is provided.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxClusters")]
    pub max_clusters: Option<i64>,
    /// Placement is an explicit list of clusters used to select member clusters to propagate resources to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<Vec<ClusterPropagationPolicyPlacement>>,
    /// ReplicasStrategy is the strategy used for scheduling replicas.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicasStrategy")]
    pub replicas_strategy: Option<ClusterPropagationPolicyReplicasStrategy>,
    /// Configures behaviors related to rescheduling.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reschedulePolicy")]
    pub reschedule_policy: Option<ClusterPropagationPolicyReschedulePolicy>,
    /// SchedulingMode determines the mode used for scheduling.
    #[serde(rename = "schedulingMode")]
    pub scheduling_mode: ClusterPropagationPolicySchedulingMode,
    /// Profile determines the scheduling profile to be used for scheduling
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "schedulingProfile")]
    pub scheduling_profile: Option<String>,
    /// Tolerations describe a set of cluster taints that the policy tolerates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<ClusterPropagationPolicyTolerations>>,
}

/// Configures behaviors related to auto migration. If absent, auto migration will be disabled.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyAutoMigration {
    /// Besides starting new replicas in other cluster(s), whether to keep the unschedulable replicas
    /// in the original cluster so we can go back to the desired state when the cluster recovers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "keepUnschedulableReplicas")]
    pub keep_unschedulable_replicas: Option<bool>,
    /// When a replica should be subject to auto migration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub when: Option<ClusterPropagationPolicyAutoMigrationWhen>,
}

/// When a replica should be subject to auto migration.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyAutoMigrationWhen {
    /// A pod will be subject to auto migration if it remains unschedulable beyond this duration.
    /// Duration should be specified in a format that can be parsed by Go's time.ParseDuration.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podUnschedulableFor")]
    pub pod_unschedulable_for: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyClusterAffinity {
    /// A list of cluster selector requirements by cluster labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<ClusterPropagationPolicyClusterAffinityMatchExpressions>>,
    /// A list of cluster selector requirements by cluster fields.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<ClusterPropagationPolicyClusterAffinityMatchFields>>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterPropagationPolicyClusterAffinityMatchExpressions {
    pub key: String,
    /// ClusterSelectorOperator is the set of operators that can be used in a cluster selector requirement.
    pub operator: ClusterPropagationPolicyClusterAffinityMatchExpressionsOperator,
    pub values: Vec<String>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPropagationPolicyClusterAffinityMatchExpressionsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ClusterPropagationPolicyClusterAffinityMatchFields {
    pub key: String,
    /// ClusterSelectorOperator is the set of operators that can be used in a cluster selector requirement.
    pub operator: ClusterPropagationPolicyClusterAffinityMatchFieldsOperator,
    pub values: Vec<String>,
}

/// ClusterSelectorRequirement is a selector that contains values, a key, and an operator that relates the values and keys
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPropagationPolicyClusterAffinityMatchFieldsOperator {
    In,
    NotIn,
    Exists,
    DoesNotExist,
    Gt,
    Lt,
}

/// DesiredPlacement describes a cluster that a federated object can be propagated to and its propagation preferences.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyPlacement {
    /// Cluster is the name of the FederatedCluster to propagate to.
    pub cluster: String,
    /// Preferences contains the cluster's propagation preferences.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferences: Option<ClusterPropagationPolicyPlacementPreferences>,
}

/// Preferences contains the cluster's propagation preferences.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyPlacementPreferences {
    /// Maximum number of replicas that should be assigned to this cluster workload object.
    /// Unbounded if no value provided (default).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxReplicas")]
    pub max_replicas: Option<i64>,
    /// Minimum number of replicas that should be assigned to this cluster workload object. 0 by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minReplicas")]
    pub min_replicas: Option<i64>,
    /// A number expressing the priority of the cluster.
    /// The higher the value, the higher the priority.
    /// When selecting clusters for propagation, clusters with higher priority are preferred.
    /// When the Binpack ReplicasStrategy is selected, replicas will be scheduled to clusters with higher priority first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// A number expressing the preference to put an additional replica to this cluster workload object.
    /// It will not take effect when ReplicasStrategy is Binpack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPropagationPolicyReplicasStrategy {
    Binpack,
    Spread,
}

/// Configures behaviors related to rescheduling.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyReschedulePolicy {
    /// DisableRescheduling determines if a federated object can be rescheduled.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "disableRescheduling")]
    pub disable_rescheduling: Option<bool>,
    /// Configures behaviors related to replica rescheduling.
    /// Default set via a post-generation patch.
    /// See patch file for details.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replicaRescheduling")]
    pub replica_rescheduling: Option<ClusterPropagationPolicyReschedulePolicyReplicaRescheduling>,
    /// When the related objects should be subject to reschedule.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rescheduleWhen")]
    pub reschedule_when: Option<ClusterPropagationPolicyReschedulePolicyRescheduleWhen>,
}

/// Configures behaviors related to replica rescheduling.
/// Default set via a post-generation patch.
/// See patch file for details.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyReschedulePolicyReplicaRescheduling {
    /// If set to true, the scheduler will attempt to prevent migrating existing replicas during rescheduling.
    /// In order to do so, replica scheduling preferences might not be fully respected.
    /// If set to false, the scheduler will always rebalance the replicas based on the specified preferences, which might
    /// cause temporary service disruption.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "avoidDisruption")]
    pub avoid_disruption: Option<bool>,
}

/// When the related objects should be subject to reschedule.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyReschedulePolicyRescheduleWhen {
    /// If set to true, changes to clusters' enabled list of api resources will trigger rescheduling.
    /// It set to false, the scheduler will reschedule only when other options are triggered or the replicas or the
    /// requested resources of the template changed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterAPIResourcesChanged")]
    pub cluster_api_resources_changed: Option<bool>,
    /// If set to true, clusters joining the federation will trigger rescheduling.
    /// It set to false, the scheduler will reschedule only when other options are triggered or the replicas or the
    /// requested resources of the template changed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterJoined")]
    pub cluster_joined: Option<bool>,
    /// If set to true, changes to cluster labels will trigger rescheduling.
    /// It set to false, the scheduler will reschedule only when other options are triggered or the replicas or the
    /// requested resources of the template changed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterLabelsChanged")]
    pub cluster_labels_changed: Option<bool>,
    /// If set to true, the scheduler will trigger rescheduling when the semantics of the policy changes. For example,
    /// modifying placement, schedulingMode, maxClusters, clusterSelector, and other configurations related to
    /// scheduling (includes reschedulePolicy itself) will immediately trigger rescheduling. Modifying the labels,
    /// annotations, autoMigration configuration will not trigger rescheduling.
    /// It set to false, the scheduler will not reschedule when the policy content changes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyContentChanged")]
    pub policy_content_changed: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ClusterPropagationPolicySchedulingMode {
    Duplicate,
    Divide,
}

/// The pod this Toleration is attached to tolerates any taint that matches
/// the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects.
    /// When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys.
    /// If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value.
    /// Valid operators are Exists and Equal. Defaults to Equal.
    /// Exists is equivalent to wildcard for value, so that a pod can
    /// tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be
    /// of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default,
    /// it is not set, which means tolerate the taint forever (do not evict). Zero and
    /// negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to.
    /// If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refCount")]
    pub ref_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "typedRefCount")]
    pub typed_ref_count: Option<Vec<ClusterPropagationPolicyStatusTypedRefCount>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPropagationPolicyStatusTypedRefCount {
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub resource: String,
}

