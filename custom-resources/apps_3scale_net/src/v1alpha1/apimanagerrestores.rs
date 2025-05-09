// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/3scale/3scale-operator/apps.3scale.net/v1alpha1/apimanagerrestores.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// APIManagerRestoreSpec defines the desired state of APIManagerRestore
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "apps.3scale.net", version = "v1alpha1", kind = "APIManagerRestore", plural = "apimanagerrestores")]
#[kube(namespaced)]
#[kube(status = "APIManagerRestoreStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct APIManagerRestoreSpec {
    /// APIManagerRestoreSource defines the backup data restore source
    /// configurability. It is a union type. Only one of the fields can be
    /// set
    #[serde(rename = "restoreSource")]
    pub restore_source: APIManagerRestoreRestoreSource,
}

/// APIManagerRestoreSource defines the backup data restore source
/// configurability. It is a union type. Only one of the fields can be
/// set
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerRestoreRestoreSource {
    /// Restore data soure configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "persistentVolumeClaim")]
    pub persistent_volume_claim: Option<APIManagerRestoreRestoreSourcePersistentVolumeClaim>,
}

/// Restore data soure configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerRestoreRestoreSourcePersistentVolumeClaim {
    /// PersistentVolumeClaim source of an existing PersistentVolumeClaim.
    /// See
    #[serde(rename = "claimSource")]
    pub claim_source: APIManagerRestoreRestoreSourcePersistentVolumeClaimClaimSource,
}

/// PersistentVolumeClaim source of an existing PersistentVolumeClaim.
/// See
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerRestoreRestoreSourcePersistentVolumeClaimClaimSource {
    /// claimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume.
    /// More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
    #[serde(rename = "claimName")]
    pub claim_name: String,
    /// readOnly Will force the ReadOnly setting in VolumeMounts.
    /// Default false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnly")]
    pub read_only: Option<bool>,
}

/// APIManagerRestoreStatus defines the observed state of APIManagerRestore
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerRestoreStatus {
    /// Name of the APIManager to be restored
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiManagerToRestoreRef")]
    pub api_manager_to_restore_ref: Option<APIManagerRestoreStatusApiManagerToRestoreRef>,
    /// Set to true when backup has been completed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    /// Restore completion time. It is represented in RFC3339 form and is in UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTime")]
    pub completion_time: Option<String>,
    /// Set to true when main steps have been completed. At this point
    /// restore still cannot be considered fully completed due to some remaining
    /// post-backup tasks are pending (cleanup, ...)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mainStepsCompleted")]
    pub main_steps_completed: Option<bool>,
    /// Restore start time. It is represented in RFC3339 form and is in UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

/// Name of the APIManager to be restored
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct APIManagerRestoreStatusApiManagerToRestoreRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

