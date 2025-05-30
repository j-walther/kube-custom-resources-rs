// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/snapp-incubator/ceph-s3-operator/s3.snappcloud.io/v1alpha1/s3buckets.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// S3BucketSpec defines the desired state of S3Bucket
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "s3.snappcloud.io", version = "v1alpha1", kind = "S3Bucket", plural = "s3buckets")]
#[kube(namespaced)]
#[kube(status = "S3BucketStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct S3BucketSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3DeletionPolicy")]
    pub s3_deletion_policy: Option<S3BucketS3DeletionPolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "s3SubuserBinding")]
    pub s3_subuser_binding: Option<Vec<S3BucketS3SubuserBinding>>,
    #[serde(rename = "s3UserRef")]
    pub s3_user_ref: String,
}

/// S3BucketSpec defines the desired state of S3Bucket
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum S3BucketS3DeletionPolicy {
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "retain")]
    Retain,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct S3BucketS3SubuserBinding {
    /// access of the subuser which can be read or write
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<S3BucketS3SubuserBindingAccess>,
    /// name of the subuser
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum S3BucketS3SubuserBindingAccess {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
}

/// S3BucketStatus defines the observed state of S3Bucket
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct S3BucketStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

