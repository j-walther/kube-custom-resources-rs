// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/kueue/kueue.x-k8s.io/v1alpha1/multikueueconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// MultiKueueConfigSpec defines the desired state of MultiKueueConfig
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kueue.x-k8s.io", version = "v1alpha1", kind = "MultiKueueConfig", plural = "multikueueconfigs")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MultiKueueConfigSpec {
    /// List of MultiKueueClusters names where the workloads from the ClusterQueue should be distributed.
    pub clusters: Vec<String>,
}

