// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/emissary-ingress/emissary/getambassador.io/v3alpha1/consulresolvers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ConsulResolver tells Ambassador to use Consul to resolve services. In addition
/// to the AmbassadorID, it needs information about which Consul server and DC to
/// use.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "getambassador.io", version = "v3alpha1", kind = "ConsulResolver", plural = "consulresolvers")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ConsulResolverSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// AmbassadorID declares which Ambassador instances should pay
    /// attention to this resource. If no value is provided, the default is:
    /// 
    /// 	ambassador_id:
    /// 	- "default"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambassador_id: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datacenter: Option<String>,
}

