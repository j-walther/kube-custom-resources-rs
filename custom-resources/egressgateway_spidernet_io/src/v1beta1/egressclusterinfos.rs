// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/spidernet-io/egressgateway/egressgateway.spidernet.io/v1beta1/egressclusterinfos.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "egressgateway.spidernet.io", version = "v1beta1", kind = "EgressClusterInfo", plural = "egressclusterinfos")]
#[kube(status = "EgressClusterInfoStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct EgressClusterInfoSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "autoDetect")]
    pub auto_detect: Option<EgressClusterInfoAutoDetect>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraCidr")]
    pub extra_cidr: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EgressClusterInfoAutoDetect {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterIP")]
    pub cluster_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeIP")]
    pub node_ip: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podCidrMode")]
    pub pod_cidr_mode: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EgressClusterInfoStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterIP")]
    pub cluster_ip: Option<EgressClusterInfoStatusClusterIp>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraCidr")]
    pub extra_cidr: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nodeIP")]
    pub node_ip: Option<BTreeMap<String, EgressClusterInfoStatusNodeIp>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podCIDR")]
    pub pod_cidr: Option<BTreeMap<String, EgressClusterInfoStatusPodCidr>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podCidrMode")]
    pub pod_cidr_mode: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EgressClusterInfoStatusClusterIp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EgressClusterInfoStatusNodeIp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct EgressClusterInfoStatusPodCidr {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv4: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ipv6: Option<Vec<String>>,
}

