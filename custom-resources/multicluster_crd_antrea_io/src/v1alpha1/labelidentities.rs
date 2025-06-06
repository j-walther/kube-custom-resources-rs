// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/antrea-io/antrea/multicluster.crd.antrea.io/v1alpha1/labelidentities.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "multicluster.crd.antrea.io", version = "v1alpha1", kind = "LabelIdentity", plural = "labelidentities")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct LabelIdentitySpec {
    /// ID is the ID allocated for the label identity by the leader cluster.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Label is the normalized string of a label identity.
    /// The format of normalized label identity is `ns:(?P<nslabels>(.)*)&pod:(?P<podlabels>(.)*)`
    /// E.g., `ns:kubernetes.io/metadata.name=kube-system&pod:app=db`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

