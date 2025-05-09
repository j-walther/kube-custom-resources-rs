// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kumahq/kuma/kuma.io/v1alpha1/containerpatches.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ContainerPatchSpec specifies the options available for a ContainerPatch
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "ContainerPatch", plural = "containerpatches")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ContainerPatchSpec {
    /// InitPatch specifies jsonpatch to apply to an init container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initPatch")]
    pub init_patch: Option<Vec<ContainerPatchInitPatch>>,
    /// SidecarPatch specifies jsonpatch to apply to a sidecar container.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sidecarPatch")]
    pub sidecar_patch: Option<Vec<ContainerPatchSidecarPatch>>,
}

/// JsonPatchBlock is one json patch operation block.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContainerPatchInitPatch {
    /// From is a jsonpatch from string, used by move and copy operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Op is a jsonpatch operation string.
    pub op: ContainerPatchInitPatchOp,
    /// Path is a jsonpatch path string.
    pub path: String,
    /// Value must be a string representing a valid json object used
    /// by replace and add operations. String has to be escaped with " to be valid a json object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// JsonPatchBlock is one json patch operation block.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ContainerPatchInitPatchOp {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "copy")]
    Copy,
}

/// JsonPatchBlock is one json patch operation block.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContainerPatchSidecarPatch {
    /// From is a jsonpatch from string, used by move and copy operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Op is a jsonpatch operation string.
    pub op: ContainerPatchSidecarPatchOp,
    /// Path is a jsonpatch path string.
    pub path: String,
    /// Value must be a string representing a valid json object used
    /// by replace and add operations. String has to be escaped with " to be valid a json object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// JsonPatchBlock is one json patch operation block.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ContainerPatchSidecarPatchOp {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "move")]
    Move,
    #[serde(rename = "copy")]
    Copy,
}

