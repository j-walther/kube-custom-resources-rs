// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws/eks-anywhere/anywhere.eks.amazonaws.com/v1alpha1/snowdatacenterconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// SnowDatacenterConfigSpec defines the desired state of SnowDatacenterConfig.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "anywhere.eks.amazonaws.com", version = "v1alpha1", kind = "SnowDatacenterConfig", plural = "snowdatacenterconfigs")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct SnowDatacenterConfigSpec {
    /// IdentityRef is a reference to an identity for the Snow API to be used when reconciling this cluster
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "identityRef")]
    pub identity_ref: Option<SnowDatacenterConfigIdentityRef>,
}

/// IdentityRef is a reference to an identity for the Snow API to be used when reconciling this cluster
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowDatacenterConfigIdentityRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SnowDatacenterConfigStatus defines the observed state of SnowDatacenterConfig.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct SnowDatacenterConfigStatus {
}

