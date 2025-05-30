// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api-provider-kubevirt/infrastructure.cluster.x-k8s.io/v1alpha1/kubevirtclusters.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// KubevirtClusterSpec defines the desired state of KubevirtCluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1alpha1", kind = "KubevirtCluster", plural = "kubevirtclusters")]
#[kube(namespaced)]
#[kube(status = "KubevirtClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct KubevirtClusterSpec {
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<KubevirtClusterControlPlaneEndpoint>,
    /// ControlPlaneServiceTemplate can be used to modify service that fronts the control plane nodes to handle the
    /// api-server traffic (port 6443). This field is optional, by default control plane nodes will use a service
    /// of type ClusterIP, which will make workload cluster only accessible within the same cluster. Note, this does
    /// not aim to expose the entire Service spec to users, but only provides capability to modify the service metadata
    /// and the service type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneServiceTemplate")]
    pub control_plane_service_template: Option<KubevirtClusterControlPlaneServiceTemplate>,
    /// InfraClusterSecretRef is a reference to a secret with a kubeconfig for external cluster used for infra.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "infraClusterSecretRef")]
    pub infra_cluster_secret_ref: Option<ObjectReference>,
    /// SSHKeys is a reference to a local struct for SSH keys persistence.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshKeys")]
    pub ssh_keys: Option<KubevirtClusterSshKeys>,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterControlPlaneEndpoint {
    /// Host is the hostname on which the API server is serving.
    pub host: String,
    /// Port is the port on which the API server is serving.
    pub port: i64,
}

/// ControlPlaneServiceTemplate can be used to modify service that fronts the control plane nodes to handle the
/// api-server traffic (port 6443). This field is optional, by default control plane nodes will use a service
/// of type ClusterIP, which will make workload cluster only accessible within the same cluster. Note, this does
/// not aim to expose the entire Service spec to users, but only provides capability to modify the service metadata
/// and the service type.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterControlPlaneServiceTemplate {
    /// Service metadata allows to set labels, annotations and namespace for the service.
    /// When infraClusterSecretRef is used, ControlPlaneService take the kubeconfig namespace by default if metadata.namespace is not specified.
    /// This field is optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, serde_json::Value>>,
    /// Service specification allows to override some fields in the service spec.
    /// Note, it does not aim cover all fields of the service spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<KubevirtClusterControlPlaneServiceTemplateSpec>,
}

/// Service specification allows to override some fields in the service spec.
/// Note, it does not aim cover all fields of the service spec.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterControlPlaneServiceTemplateSpec {
    /// Type determines how the Service is exposed. Defaults to ClusterIP. Valid
    /// options are ExternalName, ClusterIP, NodePort, and LoadBalancer.
    /// More info: https://kubernetes.io/docs/concepts/services-networking/service/#publishing-services-service-types
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
}

/// InfraClusterSecretRef is a reference to a secret with a kubeconfig for external cluster used for infra.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterInfraClusterSecretRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// SSHKeys is a reference to a local struct for SSH keys persistence.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterSshKeys {
    /// ConfigRef is a reference to a resource containing the keys.
    /// The reference is optional to allow users/operators to specify
    /// Bootstrap.DataSecretName without the need of a controller.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configRef")]
    pub config_ref: Option<ObjectReference>,
    /// DataSecretName is the name of the secret that stores ssh keys.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataSecretName")]
    pub data_secret_name: Option<String>,
}

/// ConfigRef is a reference to a resource containing the keys.
/// The reference is optional to allow users/operators to specify
/// Bootstrap.DataSecretName without the need of a controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterSshKeysConfigRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// KubevirtClusterStatus defines the observed state of KubevirtCluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterStatus {
    /// Conditions defines current service state of the KubevirtCluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// FailureDomains don't mean much in CAPD since it's all local, but we can see how the rest of cluster API
    /// will use this if we populate it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failureDomains")]
    pub failure_domains: Option<BTreeMap<String, KubevirtClusterStatusFailureDomains>>,
    /// Ready denotes that the infrastructure is ready.
    pub ready: bool,
}

/// FailureDomains don't mean much in CAPD since it's all local, but we can see how the rest of cluster API
/// will use this if we populate it.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct KubevirtClusterStatusFailureDomains {
    /// attributes is a free form map of attributes an infrastructure provider might use or require.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<BTreeMap<String, String>>,
    /// controlPlane determines if this failure domain is suitable for use by control plane machines.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlane")]
    pub control_plane: Option<bool>,
}

