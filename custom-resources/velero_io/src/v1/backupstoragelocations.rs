// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/vmware-tanzu/velero/velero.io/v1/backupstoragelocations.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// BackupStorageLocationSpec defines the desired state of a Velero BackupStorageLocation
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "velero.io", version = "v1", kind = "BackupStorageLocation", plural = "backupstoragelocations")]
#[kube(namespaced)]
#[kube(status = "BackupStorageLocationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct BackupStorageLocationSpec {
    /// AccessMode defines the permissions for the backup storage location.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<BackupStorageLocationAccessMode>,
    /// BackupSyncPeriod defines how frequently to sync backup API objects from object storage. A value of 0 disables sync.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backupSyncPeriod")]
    pub backup_sync_period: Option<String>,
    /// Config is for provider-specific configuration fields.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<BTreeMap<String, String>>,
    /// Credential contains the credential information intended to be used with this location
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential: Option<BackupStorageLocationCredential>,
    /// Default indicates this location is the default backup storage location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,
    /// ObjectStorageLocation specifies the settings necessary to connect to a provider's object storage.
    #[serde(rename = "objectStorage")]
    pub object_storage: BackupStorageLocationObjectStorage,
    /// Provider is the provider of the backup storage.
    pub provider: String,
    /// ValidationFrequency defines how frequently to validate the corresponding object storage. A value of 0 disables validation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "validationFrequency")]
    pub validation_frequency: Option<String>,
}

/// BackupStorageLocationSpec defines the desired state of a Velero BackupStorageLocation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupStorageLocationAccessMode {
    ReadOnly,
    ReadWrite,
}

/// Credential contains the credential information intended to be used with this location
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStorageLocationCredential {
    /// The key of the secret to select from.  Must be a valid secret key.
    pub key: String,
    /// Name of the referent.
    /// This field is effectively required, but due to backwards compatibility is
    /// allowed to be empty. Instances of this type with an empty value here are
    /// almost certainly wrong.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Specify whether the Secret or its key must be defined
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
}

/// ObjectStorageLocation specifies the settings necessary to connect to a provider's object storage.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStorageLocationObjectStorage {
    /// Bucket is the bucket to use for object storage.
    pub bucket: String,
    /// CACert defines a CA bundle to use when verifying TLS connections to the provider.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCert")]
    pub ca_cert: Option<String>,
    /// Prefix is the path inside a bucket to use for Velero storage. Optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// BackupStorageLocationStatus defines the observed state of BackupStorageLocation
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct BackupStorageLocationStatus {
    /// AccessMode is an unused field.
    /// 
    /// Deprecated: there is now an AccessMode field on the Spec and this field
    /// will be removed entirely as of v2.0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "accessMode")]
    pub access_mode: Option<BackupStorageLocationStatusAccessMode>,
    /// LastSyncedRevision is the value of the `metadata/revision` file in the backup
    /// storage location the last time the BSL's contents were synced into the cluster.
    /// 
    /// Deprecated: this field is no longer updated or used for detecting changes to
    /// the location's contents and will be removed entirely in v2.0.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncedRevision")]
    pub last_synced_revision: Option<String>,
    /// LastSyncedTime is the last time the contents of the location were synced into
    /// the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastSyncedTime")]
    pub last_synced_time: Option<String>,
    /// LastValidationTime is the last time the backup store location was validated
    /// the cluster.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastValidationTime")]
    pub last_validation_time: Option<String>,
    /// Message is a message about the backup storage location's status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Phase is the current state of the BackupStorageLocation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<BackupStorageLocationStatusPhase>,
}

/// BackupStorageLocationStatus defines the observed state of BackupStorageLocation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupStorageLocationStatusAccessMode {
    ReadOnly,
    ReadWrite,
}

/// BackupStorageLocationStatus defines the observed state of BackupStorageLocation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum BackupStorageLocationStatusPhase {
    Available,
    Unavailable,
}

