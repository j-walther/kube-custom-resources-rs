// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/sns-controller/sns.services.k8s.aws/v1alpha1/topics.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// TopicSpec defines the desired state of Topic.
/// 
/// A wrapper type for the topic's Amazon Resource Name (ARN). To retrieve a
/// topic's attributes, use GetTopicAttributes.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "sns.services.k8s.aws", version = "v1alpha1", kind = "Topic", plural = "topics")]
#[kube(namespaced)]
#[kube(status = "TopicStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct TopicSpec {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applicationFailureFeedbackRoleARN")]
    pub application_failure_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applicationSuccessFeedbackRoleARN")]
    pub application_success_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "applicationSuccessFeedbackSampleRate")]
    pub application_success_feedback_sample_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "contentBasedDeduplication")]
    pub content_based_deduplication: Option<String>,
    /// The body of the policy document you want to use for this topic.
    /// 
    /// You can only add one policy per topic.
    /// 
    /// The policy must be in JSON string format.
    /// 
    /// Length Constraints: Maximum length of 30,720.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "dataProtectionPolicy")]
    pub data_protection_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "deliveryPolicy")]
    pub delivery_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fifoTopic")]
    pub fifo_topic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firehoseFailureFeedbackRoleARN")]
    pub firehose_failure_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firehoseSuccessFeedbackRoleARN")]
    pub firehose_success_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "firehoseSuccessFeedbackSampleRate")]
    pub firehose_success_feedback_sample_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpFailureFeedbackRoleARN")]
    pub http_failure_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpSuccessFeedbackRoleARN")]
    pub http_success_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "httpSuccessFeedbackSampleRate")]
    pub http_success_feedback_sample_rate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsMasterKeyID")]
    pub kms_master_key_id: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "kmsMasterKeyRef")]
    pub kms_master_key_ref: Option<TopicKmsMasterKeyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lambdaFailureFeedbackRoleARN")]
    pub lambda_failure_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lambdaSuccessFeedbackRoleARN")]
    pub lambda_success_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lambdaSuccessFeedbackSampleRate")]
    pub lambda_success_feedback_sample_rate: Option<String>,
    /// The name of the topic you want to create.
    /// 
    /// Constraints: Topic names must be made up of only uppercase and lowercase
    /// ASCII letters, numbers, underscores, and hyphens, and must be between 1 and
    /// 256 characters long.
    /// 
    /// For a FIFO (first-in-first-out) topic, the name must end with the .fifo suffix.
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
    /// type to provide more user friendly syntax for references using 'from' field
    /// Ex:
    /// APIIDRef:
    /// 
    /// 	from:
    /// 	  name: my-api
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "policyRef")]
    pub policy_ref: Option<TopicPolicyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "signatureVersion")]
    pub signature_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sqsFailureFeedbackRoleARN")]
    pub sqs_failure_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sqsSuccessFeedbackRoleARN")]
    pub sqs_success_feedback_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sqsSuccessFeedbackSampleRate")]
    pub sqs_success_feedback_sample_rate: Option<String>,
    /// The list of tags to add to a new topic.
    /// 
    /// To be able to tag a topic on creation, you must have the sns:CreateTopic
    /// and sns:TagResource permissions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TopicTags>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tracingConfig")]
    pub tracing_config: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopicKmsMasterKeyRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<TopicKmsMasterKeyRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopicKmsMasterKeyRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// AWSResourceReferenceWrapper provides a wrapper around *AWSResourceReference
/// type to provide more user friendly syntax for references using 'from' field
/// Ex:
/// APIIDRef:
/// 
/// 	from:
/// 	  name: my-api
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopicPolicyRef {
    /// AWSResourceReference provides all the values necessary to reference another
    /// k8s resource for finding the identifier(Id/ARN/Name)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<TopicPolicyRefFrom>,
}

/// AWSResourceReference provides all the values necessary to reference another
/// k8s resource for finding the identifier(Id/ARN/Name)
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopicPolicyRefFrom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// The list of tags to be added to the specified topic.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopicTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// TopicStatus defines the observed state of Topic
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopicStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<TopicStatusAckResourceMetadata>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "effectiveDeliveryPolicy")]
    pub effective_delivery_policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "topicARN")]
    pub topic_arn: Option<String>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TopicStatusAckResourceMetadata {
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

