// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/gp42/aws-auth-operator/auth.ops42.org/v1alpha1/awsauthsyncconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// AwsAuthSyncConfigSpec defines the desired state of AwsAuthSyncConfig
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "auth.ops42.org", version = "v1alpha1", kind = "AwsAuthSyncConfig", plural = "awsauthsyncconfigs")]
#[kube(namespaced)]
#[kube(status = "AwsAuthSyncConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AwsAuthSyncConfigSpec {
    /// Sync AWS IAM groups to k8s RBAC groups
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncIamGroups")]
    pub sync_iam_groups: Option<Vec<AwsAuthSyncConfigSyncIamGroups>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AwsAuthSyncConfigSyncIamGroups {
    /// Destination K8s RBAC group for synchronization
    pub dest: String,
    /// Source AWS IAM group for synchronization
    pub source: String,
}

/// AwsAuthSyncConfigStatus defines the observed state of AwsAuthSyncConfig
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AwsAuthSyncConfigStatus {
    /// Important: Run "make" to regenerate code after modifying this file
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncTime")]
    pub last_sync_time: Option<String>,
    /// one of: "Success", "Failure", "No Change"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

