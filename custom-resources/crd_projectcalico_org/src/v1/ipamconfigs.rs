// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/projectcalico/calico/crd.projectcalico.org/v1/ipamconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "crd.projectcalico.org", version = "v1", kind = "IPAMConfig", plural = "ipamconfigs")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IPAMConfigSpec {
    #[serde(rename = "autoAllocateBlocks")]
    pub auto_allocate_blocks: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxBlocksPerHost")]
    pub max_blocks_per_host: Option<i64>,
    #[serde(rename = "strictAffinity")]
    pub strict_affinity: bool,
}

