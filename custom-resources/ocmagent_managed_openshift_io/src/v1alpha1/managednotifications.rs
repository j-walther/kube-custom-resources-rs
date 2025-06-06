// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/openshift/ocm-agent-operator/ocmagent.managed.openshift.io/v1alpha1/managednotifications.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// ManagedNotificationSpec defines the desired state of ManagedNotification
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "ocmagent.managed.openshift.io", version = "v1alpha1", kind = "ManagedNotification", plural = "managednotifications")]
#[kube(namespaced)]
#[kube(status = "ManagedNotificationStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ManagedNotificationSpec {
    /// AgentConfig refers to OCM agent config fields separated
    pub notifications: Vec<ManagedNotificationNotifications>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagedNotificationNotifications {
    /// The body text of the Service Log notification when the alert is active
    #[serde(rename = "activeBody")]
    pub active_body: String,
    /// LogType is a categorization property that can be used to group service logs for aggregation and managing notification preferences.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "logType")]
    pub log_type: Option<String>,
    /// The name of the notification used to associate with an alert
    pub name: String,
    /// References useful for context or remediation - this could be links to documentation, KB articles, etc
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub references: Option<Vec<String>>,
    /// Measured in hours. The minimum time interval that must elapse between active Service Log notifications
    #[serde(rename = "resendWait")]
    pub resend_wait: i32,
    /// The body text of the Service Log notification when the alert is resolved
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resolvedBody")]
    pub resolved_body: Option<String>,
    /// The severity of the Service Log notification
    pub severity: ManagedNotificationNotificationsSeverity,
    /// The summary line of the Service Log notification
    pub summary: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManagedNotificationNotificationsSeverity {
    Debug,
    Info,
    Warning,
    Major,
    Critical,
    Error,
    Fatal,
}

/// ManagedNotificationStatus defines the observed state of ManagedNotification
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedNotificationStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "notificationRecords")]
    pub notification_records: Option<Vec<ManagedNotificationStatusNotificationRecords>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagedNotificationStatusNotificationRecords {
    /// Conditions is a set of Condition instances.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<ManagedNotificationStatusNotificationRecordsConditions>>,
    /// Name of the notification
    pub name: String,
    /// ServiceLogSentCount records the number of service logs sent for the notification
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceLogSentCount")]
    pub service_log_sent_count: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagedNotificationStatusNotificationRecordsConditions {
    /// Last time the condition transit from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// (brief) reason for the condition's last transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Status of condition, one of True, False, Unknown
    pub status: String,
    /// Type of Notification condition
    #[serde(rename = "type")]
    pub r#type: ManagedNotificationStatusNotificationRecordsConditionsType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManagedNotificationStatusNotificationRecordsConditionsType {
    AlertFiring,
    AlertResolved,
    ServiceLogSent,
}

