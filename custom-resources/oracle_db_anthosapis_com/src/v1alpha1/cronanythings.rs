// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/GoogleCloudPlatform/elcarro-oracle-operator/oracle.db.anthosapis.com/v1alpha1/cronanythings.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
}
use self::prelude::*;

/// CronAnythingSpec defines the desired state of CronAnything.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "oracle.db.anthosapis.com", version = "v1alpha1", kind = "CronAnything", plural = "cronanythings")]
#[kube(namespaced)]
#[kube(status = "CronAnythingStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct CronAnythingSpec {
    /// CascadeDelete tells CronAnything to set up owner references from the created resources to the CronAnything resource. This means that if the CronAnything resource is deleted, all resources created by it will also be deleted. This is an optional field that defaults to false.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cascadeDelete")]
    pub cascade_delete: Option<bool>,
    /// ConcurrencyPolicy specifies how to treat concurrent resources if the resource provides a status path that exposes completion. The default policy if not provided is to allow a new resource to be created even if an active resource already exists. If the resource doesn’t have an active/completed status, the only supported concurrency policy is to allow creating new resources. This field is mutable. If the policy is changed to a more stringent policy while multiple resources are active, it will not delete any existing resources. The exception is if a creation of a new resource is triggered and the policy has been changed to Replace. If multiple resources are active, they will all be deleted and replaced by a new resource.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "concurrencyPolicy")]
    pub concurrency_policy: Option<String>,
    /// FinishableStrategy defines how the CronAnything controller an decide if a resource has completed. Some resources will do some work after they have been created and at some point be finished. Jobs are the most common example. If no strategy is defined, it is assumed that the resources never finish.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "finishableStrategy")]
    pub finishable_strategy: Option<CronAnythingFinishableStrategy>,
    /// ResourceBaseName specifies the base name for the resources created by CronAnything, which will be named using the format <ResourceBaseName>-<Timestamp>. This field is optional, and the default is to use the name of the CronAnything resource as the ResourceBaseName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceBaseName")]
    pub resource_base_name: Option<String>,
    /// ResourceTimestampFormat defines the format of the timestamp in the name of Resources created by CronAnything <ResourceBaseName>-<Timestamp>. This field is optional, and the default is to format the timestamp as unix time. If provided, it must be compatible with time.Format in golang.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceTimestampFormat")]
    pub resource_timestamp_format: Option<String>,
    /// Retention defines the retention policy for resources created by CronAnything. If no retention policy is defined, CronAnything will never delete resources, so cleanup must be handled through some other process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<CronAnythingRetention>,
    /// Schedule defines a time-based schedule, e.g., a standard cron schedule such as “@every 10m”. This field is mandatory and mutable. If it is changed, resources will simply be created at the new interval from then on.
    pub schedule: String,
    /// Suspend tells the controller to suspend creation of additional resources. The default value is false. This field is mutable. It will not affect any existing resources, but only affect creation of additional resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// Template is a template of a resource type for which instances are to be created on the given schedule. This field is mandatory and it must contain a valid template for an existing apiVersion and kind in the cluster. It is immutable, so if the template needs to change, the whole CronAnything resource should be replaced.
    pub template: BTreeMap<String, serde_json::Value>,
    /// TotalResourceLimit specifies the total number of children allowed for a particular CronAnything resource. If this limit is reached, no additional resources will be created. This limit is mostly meant to avoid runaway creation of resources that could bring down the cluster. Both finished and unfinished resources count against this limit. This field is mutable. If it is changed to a lower value than the existing number of resources, none of the existing resources will be deleted as a result, but no additional resources will be created until the number of child resources goes below the limit. The field is optional with a default value of 100.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "totalResourceLimit")]
    pub total_resource_limit: Option<i32>,
    /// TriggerDeadlineSeconds defines Deadline in seconds for creating the resource if it missed the scheduled time. If no deadline is provided, the resource will be created no matter how far after the scheduled time. If multiple triggers were missed, only the last will be triggered and only one resource will be created. This field is mutable and changing it will affect the creation of new resources from that point in time.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerDeadlineSeconds")]
    pub trigger_deadline_seconds: Option<i64>,
}

/// FinishableStrategy defines how the CronAnything controller an decide if a resource has completed. Some resources will do some work after they have been created and at some point be finished. Jobs are the most common example. If no strategy is defined, it is assumed that the resources never finish.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingFinishableStrategy {
    /// StringField contains the details for how the CronAnything controller can find the string field on the resource needed to decide if the resource has completed. It also lists the values that mean the resource has completed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stringField")]
    pub string_field: Option<CronAnythingFinishableStrategyStringField>,
    /// TimestampField contains the details for how the CronAnything controller can find the timestamp field on the resource in order to decide if the resource has completed.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "timestampField")]
    pub timestamp_field: Option<CronAnythingFinishableStrategyTimestampField>,
    /// Type tells which strategy should be used.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// StringField contains the details for how the CronAnything controller can find the string field on the resource needed to decide if the resource has completed. It also lists the values that mean the resource has completed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingFinishableStrategyStringField {
    /// The path to the field on the resource that contains a string value.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
    /// The values of the field that means the resource has completed.
    #[serde(rename = "finishedValues")]
    pub finished_values: Vec<String>,
}

/// TimestampField contains the details for how the CronAnything controller can find the timestamp field on the resource in order to decide if the resource has completed.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingFinishableStrategyTimestampField {
    /// The path to the field on the resource that contains the timestamp.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// Retention defines the retention policy for resources created by CronAnything. If no retention policy is defined, CronAnything will never delete resources, so cleanup must be handled through some other process.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingRetention {
    /// The number of completed resources to keep before deleting them. This only affects finishable resources and the default value is 3. This field is mutable and if it is changed to a number lower than the current number of finished resources, the oldest ones will eventually be deleted until the number of finished resources matches the limit.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "historyCountLimit")]
    pub history_count_limit: Option<i32>,
    /// The time since completion that a resource is kept before deletion. This only affects finishable resources. This does not have any default value and if it is not provided, HistoryCountLimit will be used to prune completed resources. If both HistoryCountLimit and  HistoryTimeLimitSeconds are set, it is treated as an OR operation.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "historyTimeLimitSeconds")]
    pub history_time_limit_seconds: Option<i64>,
    /// ResourceTimestampStrategy specifies how the CronAnything controller can find the age of a resource. This is needed to support retention.
    #[serde(rename = "resourceTimestampStrategy")]
    pub resource_timestamp_strategy: CronAnythingRetentionResourceTimestampStrategy,
}

/// ResourceTimestampStrategy specifies how the CronAnything controller can find the age of a resource. This is needed to support retention.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingRetentionResourceTimestampStrategy {
    /// FieldResourceTimestampStrategy specifies how the CronAnything controller can find the timestamp for the resource from a field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<CronAnythingRetentionResourceTimestampStrategyField>,
    /// Type tells which strategy should be used.
    #[serde(rename = "type")]
    pub r#type: String,
}

/// FieldResourceTimestampStrategy specifies how the CronAnything controller can find the timestamp for the resource from a field.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingRetentionResourceTimestampStrategyField {
    /// The path to the field on the resource that contains the timestamp.
    #[serde(rename = "fieldPath")]
    pub field_path: String,
}

/// CronAnythingStatus defines the observed state of CronAnything.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingStatus {
    /// LastScheduleTime keeps track of the scheduled time for the last successfully completed creation of a resource. This is used by the controller to determine when the next resource creation should happen. If creation of a resource is delayed for any reason but eventually does happen, this value will still be updated to the time when it was originally scheduled to happen.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastScheduleTime")]
    pub last_schedule_time: Option<String>,
    /// PendingTrigger keeps track of any triggers that are past their trigger time, but for some reason have not been completed yet. This is typically a result of the create operation failing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pendingTrigger")]
    pub pending_trigger: Option<CronAnythingStatusPendingTrigger>,
    /// TriggerHistory keeps track of the status for the last 10 triggers. This allows users of CronAnything to see whether any triggers failed. It is important to know that this only keeps track of whether a trigger was successfully executed (as in creating the given resource), not whether the created resource was itself successful. For this information, any users of CronAnything should observe the resources created.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "triggerHistory")]
    pub trigger_history: Option<Vec<CronAnythingStatusTriggerHistory>>,
}

/// PendingTrigger keeps track of any triggers that are past their trigger time, but for some reason have not been completed yet. This is typically a result of the create operation failing.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingStatusPendingTrigger {
    /// Result tells why this trigger is in the pending state, i.e. what prevented it from completing successfully.
    pub result: String,
    /// ScheduleTime is the time when this trigger was scheduled to be executed.
    #[serde(rename = "scheduleTime")]
    pub schedule_time: String,
}

/// TriggerHistoryRecord contains information about the result of a trigger. It can either have completed successfully, and if it did not, the record will provide information about what is the cause of the failure.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct CronAnythingStatusTriggerHistory {
    /// CreationTimestamp is the time when this record was created. This is thus also the time at which the final result of the trigger was decided.
    #[serde(rename = "creationTimestamp")]
    pub creation_timestamp: String,
    /// Result contains the outcome of a trigger. It can either be CreateSucceeded, which means the given resource was created as intended, or it can be one of several error messages.
    pub result: String,
    /// ScheduleTime is the time when this trigger was scheduled to be executed.
    #[serde(rename = "scheduleTime")]
    pub schedule_time: String,
}

