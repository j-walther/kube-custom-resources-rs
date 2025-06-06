// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/ipfs-cluster/ipfs-operator/cluster.ipfs.io/v1alpha1/ipfsclusters.yaml
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

/// IpfsClusterSpec defines the desired state of the IpfsCluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cluster.ipfs.io", version = "v1alpha1", kind = "IpfsCluster", plural = "ipfsclusters")]
#[kube(namespaced)]
#[kube(status = "IpfsClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IpfsClusterSpec {
    /// clusterStorage defines the amount of storage to be used by IPFS Cluster.
    #[serde(rename = "clusterStorage")]
    pub cluster_storage: IntOrString,
    /// follows defines the list of other IPFS Clusters this one should follow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follows: Option<Vec<IpfsClusterFollows>>,
    /// ipfsResources specifies the resource requirements for each IPFS container. If this value is omitted, then the operator will automatically determine these settings based on the storage sizes used.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ipfsResources")]
    pub ipfs_resources: Option<IpfsClusterIpfsResources>,
    /// ipfsStorage defines the total storage to be allocated by this resource.
    #[serde(rename = "ipfsStorage")]
    pub ipfs_storage: IntOrString,
    /// networking defines network configuration settings.
    pub networking: IpfsClusterNetworking,
    /// replicas sets the number of replicas of IPFS Cluster nodes we should be running.
    pub replicas: i32,
    /// reprovider Describes the settings that each IPFS node should use when reproviding content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reprovider: Option<IpfsClusterReprovider>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IpfsClusterFollows {
    pub name: String,
    pub template: String,
}

/// ipfsResources specifies the resource requirements for each IPFS container. If this value is omitted, then the operator will automatically determine these settings based on the storage sizes used.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IpfsClusterIpfsResources {
    /// Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// networking defines network configuration settings.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IpfsClusterNetworking {
    /// circuitRelays defines how many CircuitRelays should be created.
    #[serde(rename = "circuitRelays")]
    pub circuit_relays: i32,
    /// public is a switch which defines whether this IPFSCluster will use the global IPFS network or create its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}

/// reprovider Describes the settings that each IPFS node should use when reproviding content.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IpfsClusterReprovider {
    /// Interval sets the time between rounds of reproviding local content to the routing system. Defaults to '12h'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// Strategy specifies the reprovider strategy, defaults to 'all'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<IpfsClusterReproviderStrategy>,
}

/// reprovider Describes the settings that each IPFS node should use when reproviding content.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum IpfsClusterReproviderStrategy {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "pinned")]
    Pinned,
    #[serde(rename = "roots")]
    Roots,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IpfsClusterStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "circuitRelays")]
    pub circuit_relays: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

