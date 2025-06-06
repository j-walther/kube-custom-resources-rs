// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubean-io/kubean/kubean.io/v1alpha1/clusters.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// Spec defines the desired state of a member cluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kubean.io", version = "v1alpha1", kind = "Cluster", plural = "clusters")]
#[kube(status = "ClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ClusterSpec {
    /// HostsConfRef stores hosts.yml.
    #[serde(rename = "hostsConfRef")]
    pub hosts_conf_ref: ClusterHostsConfRef,
    /// KubeConfRef stores cluster kubeconfig.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kubeconfRef")]
    pub kubeconf_ref: Option<ClusterKubeconfRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "preCheckRef")]
    pub pre_check_ref: Option<ClusterPreCheckRef>,
    /// SSHAuthRef stores ssh key and if it is empty ,then use sshpass.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sshAuthRef")]
    pub ssh_auth_ref: Option<ClusterSshAuthRef>,
    /// VarsConfRef stores group_vars.yml.
    #[serde(rename = "varsConfRef")]
    pub vars_conf_ref: ClusterVarsConfRef,
}

/// HostsConfRef stores hosts.yml.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterHostsConfRef {
    pub name: String,
    pub namespace: String,
}

/// KubeConfRef stores cluster kubeconfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterKubeconfRef {
    pub name: String,
    pub namespace: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterPreCheckRef {
    pub name: String,
    pub namespace: String,
}

/// SSHAuthRef stores ssh key and if it is empty ,then use sshpass.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterSshAuthRef {
    pub name: String,
    pub namespace: String,
}

/// VarsConfRef stores group_vars.yml.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterVarsConfRef {
    pub name: String,
    pub namespace: String,
}

/// Status contains information about the current status of a cluster updated periodically by cluster controller.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatus {
    pub conditions: Vec<ClusterStatusConditions>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ClusterStatusConditions {
    /// ClusterOps refers to the name of ClusterOperation.
    #[serde(rename = "clusterOps")]
    pub cluster_ops: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

