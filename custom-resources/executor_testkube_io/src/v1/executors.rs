// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubeshop/testkube-operator/executor.testkube.io/v1/executors.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// ExecutorSpec defines the desired state of Executor
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "executor.testkube.io", version = "v1", kind = "Executor", plural = "executors")]
#[kube(namespaced)]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ExecutorSpec {
    /// executor binary arguments
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// executor default binary command
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// ContentTypes list of handled content types
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<String>>,
    /// ExecutorType one of "rest" for rest openapi based executors or "job" which will be default runners for testkube
    /// or "container" for container executors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executor_type: Option<ExecutorExecutorType>,
    /// Features list of possible features which executor handles
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    /// Image for kube-job
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// container executor default image pull secrets
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "imagePullSecrets")]
    pub image_pull_secrets: Option<Vec<ExecutorImagePullSecrets>>,
    /// name of the template resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jobTemplateReference")]
    pub job_template_reference: Option<String>,
    /// Job template to launch executor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_template: Option<String>,
    /// Meta data about executor
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<ExecutorMeta>,
    /// Slaves data to run test in distributed environment
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slaves: Option<ExecutorSlaves>,
    /// Types defines what types can be handled by executor e.g. "postman/collection", ":curl/command" etc
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// URI for rest based executors
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    /// use data dir as working dir for executor
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDataDirAsWorkingDir")]
    pub use_data_dir_as_working_dir: Option<bool>,
}

/// ExecutorSpec defines the desired state of Executor
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ExecutorExecutorType {
    #[serde(rename = "job")]
    Job,
    #[serde(rename = "container")]
    Container,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutorImagePullSecrets {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Meta data about executor
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutorMeta {
    /// URI for executor docs
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "docsURI")]
    pub docs_uri: Option<String>,
    /// URI for executor icon
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "iconURI")]
    pub icon_uri: Option<String>,
    /// executor tooltips
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tooltips: Option<BTreeMap<String, String>>,
}

/// Slaves data to run test in distributed environment
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutorSlaves {
    pub image: String,
}

/// ExecutorStatus defines the observed state of Executor
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutorStatus {
}

