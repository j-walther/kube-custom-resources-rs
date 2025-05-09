// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/cloudwatchlogs-controller/cloudwatchlogs.services.k8s.aws/v1alpha1/loggroups.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// LogGroupSpec defines the desired state of LogGroup.
/// 
/// Represents a log group.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "cloudwatchlogs.services.k8s.aws", version = "v1alpha1", kind = "LogGroup", plural = "loggroups")]
#[kube(namespaced)]
#[kube(status = "LogGroupStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct LogGroupSpec {
    /// The Amazon Resource Name (ARN) of the KMS key to use when encrypting log
    /// data. For more information, see Amazon Resource Names (https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kms).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyID")]
    pub kms_key_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsKeyRef")]
    pub kms_key_ref: Option<LogGroupKmsKeyRef>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retentionDays")]
    pub retention_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subscriptionFilters")]
    pub subscription_filters: Option<Vec<LogGroupSubscriptionFilters>>,
    /// The key-value pairs to use for the tags.
    /// 
    /// You can grant users access to certain log groups while preventing them from
    /// accessing other log groups. To do so, tag your groups and use IAM policies
    /// that refer to those tags. To assign tags when you create a log group, you
    /// must have either the logs:TagResource or logs:TagLogGroup permission. For
    /// more information about tagging, see Tagging Amazon Web Services resources
    /// (https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html). For more
    /// information about using tags to control access, see Controlling access to
    /// Amazon Web Services resources using tags (https://docs.aws.amazon.com/IAM/latest/UserGuide/access_tags.html).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LogGroupKmsKeyRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<LogGroupKmsKeyRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LogGroupKmsKeyRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LogGroupSubscriptionFilters {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "destinationARN")]
    pub destination_arn: Option<String>,
    /// The method used to distribute log data to the destination, which can be either
    /// random or grouped by log stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distribution: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterName")]
    pub filter_name: Option<String>,
    /// A symbolic description of how CloudWatch Logs should interpret the data in
    /// each log event. For example, a log event can contain timestamps, IP addresses,
    /// strings, and so on. You use the filter pattern to specify what to look for
    /// in the log event message.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "filterPattern")]
    pub filter_pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "roleARN")]
    pub role_arn: Option<String>,
}

/// LogGroupStatus defines the observed state of LogGroup
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LogGroupStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<LogGroupStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// The creation time of the log group, expressed as the number of milliseconds
    /// after Jan 1, 1970 00:00:00 UTC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "creationTime")]
    pub creation_time: Option<i64>,
    /// Displays whether this log group has a protection policy, or whether it had
    /// one in the past. For more information, see PutDataProtectionPolicy (https://docs.aws.amazon.com/AmazonCloudWatchLogs/latest/APIReference/API_PutDataProtectionPolicy.html).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataProtectionStatus")]
    pub data_protection_status: Option<String>,
    /// The number of metric filters.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "metricFilterCount")]
    pub metric_filter_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retentionInDays")]
    pub retention_in_days: Option<i64>,
    /// The number of bytes stored.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "storedBytes")]
    pub stored_bytes: Option<i64>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct LogGroupStatusAckResourceMetadata {
    /// ARN is the Amazon Resource Name for the resource. This is a
    /// globally-unique identifier and is set only by the ACK service controller
    /// once the controller has orchestrated the creation of the resource OR
    /// when it has verified that an "adopted" resource (a resource where the
    /// ARN annotation was set by the Kubernetes user on the CR) exists and
    /// matches the supplied CR's Spec field values.
    /// https://github.com/aws/aws-controllers-k8s/issues/270
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// OwnerAccountID is the AWS Account ID of the account that owns the
    /// backend AWS service API resource.
    #[serde(rename = "ownerAccountID")]
    pub owner_account_id: String,
    /// Region is the AWS region in which the resource exists or will exist.
    pub region: String,
}

