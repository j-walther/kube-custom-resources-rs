// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tinkerbell/rufio/bmc.tinkerbell.org/v1alpha1/jobs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// JobSpec defines the desired state of Job.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "bmc.tinkerbell.org", version = "v1alpha1", kind = "Job", plural = "jobs")]
#[kube(namespaced)]
#[kube(status = "JobStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct JobSpec {
    /// MachineRef represents the Machine resource to execute the job.
    /// All the tasks in the job are executed for the same Machine.
    #[serde(rename = "machineRef")]
    pub machine_ref: JobMachineRef,
    /// Tasks represents a list of baseboard management actions to be executed.
    /// The tasks are executed sequentially. Controller waits for one task to complete before executing the next.
    /// If a single task fails, job execution stops and sets condition Failed.
    /// Condition Completed is set only if all the tasks were successful.
    pub tasks: Vec<JobTasks>,
}

/// MachineRef represents the Machine resource to execute the job.
/// All the tasks in the job are executed for the same Machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobMachineRef {
    /// Name of the Machine.
    pub name: String,
    /// Namespace the Machine resides in.
    pub namespace: String,
}

/// Action represents the action to be performed.
/// A single task can only perform one type of action.
/// For example either PowerAction or OneTimeBootDeviceAction.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobTasks {
    /// OneTimeBootDeviceAction represents a baseboard management one time set boot device operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "oneTimeBootDeviceAction")]
    pub one_time_boot_device_action: Option<JobTasksOneTimeBootDeviceAction>,
    /// PowerAction represents a baseboard management power operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "powerAction")]
    pub power_action: Option<JobTasksPowerAction>,
    /// VirtualMediaAction represents a baseboard management virtual media insert/eject.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "virtualMediaAction")]
    pub virtual_media_action: Option<JobTasksVirtualMediaAction>,
}

/// OneTimeBootDeviceAction represents a baseboard management one time set boot device operation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobTasksOneTimeBootDeviceAction {
    /// Devices represents the boot devices, in order for setting one time boot.
    /// Currently only the first device in the slice is used to set one time boot.
    pub device: Vec<String>,
    /// EFIBoot instructs the machine to use EFI boot.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "efiBoot")]
    pub efi_boot: Option<bool>,
}

/// Action represents the action to be performed.
/// A single task can only perform one type of action.
/// For example either PowerAction or OneTimeBootDeviceAction.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum JobTasksPowerAction {
    #[serde(rename = "on")]
    On,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "soft")]
    Soft,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "cycle")]
    Cycle,
    #[serde(rename = "reset")]
    Reset,
}

/// VirtualMediaAction represents a baseboard management virtual media insert/eject.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobTasksVirtualMediaAction {
    pub kind: String,
    /// mediaURL represents the URL of the image to be inserted into the virtual media, or empty to
    /// eject media.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mediaURL")]
    pub media_url: Option<String>,
}

/// JobStatus defines the observed state of Job.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatus {
    /// CompletionTime represents time when the job was completed.
    /// The completion time is only set when the job finishes successfully.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "completionTime")]
    pub completion_time: Option<String>,
    /// Conditions represents the latest available observations of an object's current state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<JobStatusConditions>>,
    /// StartTime represents time when the Job controller started processing a job.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct JobStatusConditions {
    /// Message represents human readable message indicating details about last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status is the status of the Job condition.
    /// Can be True or False.
    pub status: String,
    /// Type of the Job condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

