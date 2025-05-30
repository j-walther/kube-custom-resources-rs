// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/node-feature-discovery-operator/nfd.kubernetes.io/v1/nodefeaturediscoveries.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// NodeFeatureDiscoverySpec defines the desired state of NodeFeatureDiscovery
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "nfd.kubernetes.io", version = "v1", kind = "NodeFeatureDiscovery", plural = "nodefeaturediscoveries")]
#[kube(namespaced)]
#[kube(status = "NodeFeatureDiscoveryStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct NodeFeatureDiscoverySpec {
    /// EnableTaints enables the enable the experimental tainting feature This allows keeping nodes with specialized hardware away from running general workload i and instead leave them for workloads that need the specialized hardware.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "enableTaints")]
    pub enable_taints: Option<bool>,
    /// ExtraLabelNs defines the list of of allowed extra label namespaces By default, only allow labels in the default `feature.node.kubernetes.io` label namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraLabelNs")]
    pub extra_label_ns: Option<Vec<String>>,
    /// Instance name. Used to separate annotation namespaces for multiple parallel deployments.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    /// LabelWhiteList defines a regular expression for filtering feature labels based on their name. Each label must match against the given reqular expression in order to be published.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "labelWhiteList")]
    pub label_white_list: Option<String>,
    /// OperandSpec describes configuration options for the operand
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operand: Option<NodeFeatureDiscoveryOperand>,
    /// PruneOnDelete defines whether the NFD-master prune should be enabled or not. If enabled, the Operator will deploy an NFD-Master prune job that will remove all NFD labels (and other NFD-managed assets such as annotations, extended resources and taints) from the cluster nodes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prunerOnDelete")]
    pub pruner_on_delete: Option<bool>,
    /// ResourceLabels defines the list of features to be advertised as extended resources instead of labels.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceLabels")]
    pub resource_labels: Option<Vec<String>>,
    /// Deploy the NFD-Topology-Updater NFD-Topology-Updater is a daemon responsible for examining allocated resources on a worker node to account for resources available to be allocated to new pod on a per-zone basis https://kubernetes-sigs.github.io/node-feature-discovery/master/get-started/introduction.html#nfd-topology-updater
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topologyUpdater")]
    pub topology_updater: Option<bool>,
    /// WorkerConfig describes configuration options for the NFD worker.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workerConfig")]
    pub worker_config: Option<NodeFeatureDiscoveryWorkerConfig>,
}

/// OperandSpec describes configuration options for the operand
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperand {
    /// Image defines the image to pull for the NFD operand [defaults to registry.k8s.io/nfd/node-feature-discovery]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// ImagePullPolicy defines Image pull policy for the NFD operand image [defaults to Always]
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullPolicy")]
    pub image_pull_policy: Option<String>,
    /// MasterEnv defines environment variables to be added to the master deployment
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterEnvs")]
    pub master_envs: Option<Vec<NodeFeatureDiscoveryOperandMasterEnvs>>,
    /// MasterTolerations defines tolerations to be applied to the master deployment
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "masterTolerations")]
    pub master_tolerations: Option<Vec<NodeFeatureDiscoveryOperandMasterTolerations>>,
    /// ServicePort specifies the TCP port that nfd-master listens for incoming requests.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "servicePort")]
    pub service_port: Option<i64>,
    /// WorkerEnv defines environment variables to be added to the worker Daemonset
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workerEnvs")]
    pub worker_envs: Option<Vec<NodeFeatureDiscoveryOperandWorkerEnvs>>,
    /// WorkerTolerations defines tolerations to be applied to the worker Daemonset
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "workerTolerations")]
    pub worker_tolerations: Option<Vec<NodeFeatureDiscoveryOperandWorkerTolerations>>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandMasterEnvs {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<NodeFeatureDiscoveryOperandMasterEnvsValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandMasterEnvsValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<NodeFeatureDiscoveryOperandMasterEnvsValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<NodeFeatureDiscoveryOperandMasterEnvsValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<NodeFeatureDiscoveryOperandMasterEnvsValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<NodeFeatureDiscoveryOperandMasterEnvsValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandMasterEnvsValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandMasterEnvsValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandMasterEnvsValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandMasterEnvsValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandMasterTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// EnvVar represents an environment variable present in a Container.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandWorkerEnvs {
    /// Name of the environment variable. Must be a C_IDENTIFIER.
    pub name: String,
    /// Variable references $(VAR_NAME) are expanded using the previously defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. Double $$ are reduced to a single $, which allows for escaping the $(VAR_NAME) syntax: i.e. "$$(VAR_NAME)" will produce the string literal "$(VAR_NAME)". Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Source for the environment variable's value. Cannot be used if value is not empty.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueFrom")]
    pub value_from: Option<NodeFeatureDiscoveryOperandWorkerEnvsValueFrom>,
}

/// Source for the environment variable's value. Cannot be used if value is not empty.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandWorkerEnvsValueFrom {
    /// Selects a key of a ConfigMap.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configMapKeyRef")]
    pub config_map_key_ref: Option<NodeFeatureDiscoveryOperandWorkerEnvsValueFromConfigMapKeyRef>,
    /// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldRef")]
    pub field_ref: Option<NodeFeatureDiscoveryOperandWorkerEnvsValueFromFieldRef>,
    /// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceFieldRef")]
    pub resource_field_ref: Option<NodeFeatureDiscoveryOperandWorkerEnvsValueFromResourceFieldRef>,
    /// Selects a key of a secret in the pod's namespace
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<NodeFeatureDiscoveryOperandWorkerEnvsValueFromSecretKeyRef>,
}

/// Selects a key of a ConfigMap.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandWorkerEnvsValueFromConfigMapKeyRef {
    /// The key to select.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the ConfigMap or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandWorkerEnvsValueFromFieldRef {
    /// Version of the schema the FieldPath is written in terms of, defaults to "v1".
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Path of the field to select in the specified API version.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandWorkerEnvsValueFromResourceFieldRef {
    /// Container name: required for volumes, optional for env vars
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containerName")]
    pub container_name: Option<String>,
    /// Specifies the output format of the exposed resources, defaults to "1"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divisor: Option<IntOrString>,
    /// Required: resource to select
    pub resource: String,
}

/// Selects a key of a secret in the pod's namespace
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandWorkerEnvsValueFromSecretKeyRef {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryOperandWorkerTolerations {
    /// Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,
    /// Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tolerationSeconds")]
    pub toleration_seconds: Option<i64>,
    /// Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// WorkerConfig describes configuration options for the NFD worker.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryWorkerConfig {
    /// BinaryData holds the NFD configuration file
    #[serde(rename = "configData")]
    pub config_data: String,
}

/// NodeFeatureDiscoveryStatus defines the observed state of NodeFeatureDiscovery
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct NodeFeatureDiscoveryStatus {
    /// Conditions represents the latest available observations of current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

