// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/rook/rook/ceph.rook.io/v1/cephcosidrivers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// Spec represents the specification of a Ceph COSI Driver
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ceph.rook.io", version = "v1", kind = "CephCOSIDriver", plural = "cephcosidrivers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CephCOSIDriverSpec {
    /// DeploymentStrategy is the strategy to use to deploy the COSI driver.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deploymentStrategy")]
    pub deployment_strategy: Option<CephCOSIDriverDeploymentStrategy>,
    /// Image is the container image to run the Ceph COSI driver
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// ObjectProvisionerImage is the container image to run the COSI driver sidecar
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "objectProvisionerImage")]
    pub object_provisioner_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placement: Option<CephCOSIDriverPlacement>,
    /// Resources is the resource requirements for the COSI driver
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<CephCOSIDriverResources>,
}

/// Spec represents the specification of a Ceph COSI Driver
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CephCOSIDriverDeploymentStrategy {
    Never,
    Auto,
    Always,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacement {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinity")]
    pub node_affinity: Option<CephCOSIDriverPlacementNodeAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podAffinity")]
    pub pod_affinity: Option<CephCOSIDriverPlacementPodAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podAntiAffinity")]
    pub pod_anti_affinity: Option<CephCOSIDriverPlacementPodAntiAffinity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerations: Option<Vec<CephCOSIDriverPlacementTolerations>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologySpreadConstraints")]
    pub topology_spread_constraints: Option<Vec<CephCOSIDriverPlacementTopologySpreadConstraints>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<CephCOSIDriverPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<CephCOSIDriverPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    pub preference: CephCOSIDriverPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreference {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<CephCOSIDriverPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinityPreferredDuringSchedulingIgnoredDuringExecutionPreferenceMatchFields {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "nodeSelectorTerms")]
    pub node_selector_terms: Vec<CephCOSIDriverPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTerms {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchFields")]
    pub match_fields: Option<Vec<CephCOSIDriverPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementNodeAffinityRequiredDuringSchedulingIgnoredDuringExecutionNodeSelectorTermsMatchFields {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<Vec<CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinity {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preferredDuringSchedulingIgnoredDuringExecution")]
    pub preferred_during_scheduling_ignored_during_execution: Option<Vec<CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requiredDuringSchedulingIgnoredDuringExecution")]
    pub required_during_scheduling_ignored_during_execution: Option<Vec<CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecution {
    #[serde(rename = "podAffinityTerm")]
    pub pod_affinity_term: CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm,
    pub weight: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTerm {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityPreferredDuringSchedulingIgnoredDuringExecutionPodAffinityTermNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecution {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mismatchLabelKeys")]
    pub mismatch_label_keys: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "namespaceSelector")]
    pub namespace_selector: Option<CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<String>>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementPodAntiAffinityRequiredDuringSchedulingIgnoredDuringExecutionNamespaceSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementTolerations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementTopologySpreadConstraints {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelSelector")]
    pub label_selector: Option<CephCOSIDriverPlacementTopologySpreadConstraintsLabelSelector>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabelKeys")]
    pub match_label_keys: Option<Vec<String>>,
    #[serde(rename = "maxSkew")]
    pub max_skew: i32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minDomains")]
    pub min_domains: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeAffinityPolicy")]
    pub node_affinity_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeTaintsPolicy")]
    pub node_taints_policy: Option<String>,
    #[serde(rename = "topologyKey")]
    pub topology_key: String,
    #[serde(rename = "whenUnsatisfiable")]
    pub when_unsatisfiable: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementTopologySpreadConstraintsLabelSelector {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchExpressions")]
    pub match_expressions: Option<Vec<CephCOSIDriverPlacementTopologySpreadConstraintsLabelSelectorMatchExpressions>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchLabels")]
    pub match_labels: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverPlacementTopologySpreadConstraintsLabelSelectorMatchExpressions {
    pub key: String,
    pub operator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// Resources is the resource requirements for the COSI driver
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// 
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// 
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<CephCOSIDriverResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CephCOSIDriverResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
    /// Request is the name chosen for a request in the referenced claim.
    /// If empty, everything from the claim is made available, otherwise
    /// only the result of this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}

