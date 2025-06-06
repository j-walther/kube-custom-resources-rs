// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/aws-controllers-k8s/route53-controller/route53.services.k8s.aws/v1alpha1/hostedzones.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// HostedZoneSpec defines the desired state of HostedZone.
/// 
/// A complex type that contains general information about the hosted zone.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "route53.services.k8s.aws", version = "v1alpha1", kind = "HostedZone", plural = "hostedzones")]
#[kube(namespaced)]
#[kube(status = "HostedZoneStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct HostedZoneSpec {
    /// If you want to associate a reusable delegation set with this hosted zone,
    /// the ID that Amazon Route 53 assigned to the reusable delegation set when
    /// you created it. For more information about reusable delegation sets, see
    /// CreateReusableDelegationSet (https://docs.aws.amazon.com/Route53/latest/APIReference/API_CreateReusableDelegationSet.html).
    /// 
    /// If you are using a reusable delegation set to create a public hosted zone
    /// for a subdomain, make sure that the parent hosted zone doesn't use one or
    /// more of the same name servers. If you have overlapping nameservers, the operation
    /// will cause a ConflictingDomainsExist error.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "delegationSetID")]
    pub delegation_set_id: Option<String>,
    /// (Optional) A complex type that contains the following optional values:
    /// 
    ///    * For public and private hosted zones, an optional comment
    /// 
    ///    * For private hosted zones, an optional PrivateZone element
    /// 
    /// If you don't specify a comment or the PrivateZone element, omit HostedZoneConfig
    /// and the other elements.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostedZoneConfig")]
    pub hosted_zone_config: Option<HostedZoneHostedZoneConfig>,
    /// The name of the domain. Specify a fully qualified domain name, for example,
    /// www.example.com. The trailing dot is optional; Amazon Route 53 assumes that
    /// the domain name is fully qualified. This means that Route 53 treats www.example.com
    /// (without a trailing dot) and www.example.com. (with a trailing dot) as identical.
    /// 
    /// If you're creating a public hosted zone, this is the name you have registered
    /// with your DNS registrar. If your domain name is registered with a registrar
    /// other than Route 53, change the name servers for your domain to the set of
    /// NameServers that CreateHostedZone returns in DelegationSet.
    pub name: String,
    /// A complex type that contains a list of the tags that you want to add to the
    /// specified health check or hosted zone and/or the tags that you want to edit
    /// Value for.
    /// 
    /// You can add a maximum of 10 tags to a health check or a hosted zone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<HostedZoneTags>>,
    /// (Private hosted zones only) A complex type that contains information about
    /// the Amazon VPC that you're associating with this hosted zone.
    /// 
    /// You can specify only one Amazon VPC when you create a private hosted zone.
    /// If you are associating a VPC with a hosted zone with this request, the paramaters
    /// VPCId and VPCRegion are also required.
    /// 
    /// To associate additional Amazon VPCs with the hosted zone, use AssociateVPCWithHostedZone
    /// (https://docs.aws.amazon.com/Route53/latest/APIReference/API_AssociateVPCWithHostedZone.html)
    /// after you create a hosted zone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<HostedZoneVpc>,
}

/// (Optional) A complex type that contains the following optional values:
/// 
///    * For public and private hosted zones, an optional comment
/// 
///    * For private hosted zones, an optional PrivateZone element
/// 
/// If you don't specify a comment or the PrivateZone element, omit HostedZoneConfig
/// and the other elements.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostedZoneHostedZoneConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateZone")]
    pub private_zone: Option<bool>,
}

/// A complex type that contains information about a tag that you want to add
/// or edit for the specified health check or hosted zone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostedZoneTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// (Private hosted zones only) A complex type that contains information about
/// the Amazon VPC that you're associating with this hosted zone.
/// 
/// You can specify only one Amazon VPC when you create a private hosted zone.
/// If you are associating a VPC with a hosted zone with this request, the paramaters
/// VPCId and VPCRegion are also required.
/// 
/// To associate additional Amazon VPCs with the hosted zone, use AssociateVPCWithHostedZone
/// (https://docs.aws.amazon.com/Route53/latest/APIReference/API_AssociateVPCWithHostedZone.html)
/// after you create a hosted zone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostedZoneVpc {
    /// (Private hosted zones only) The ID of an Amazon VPC.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcID")]
    pub vpc_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcRegion")]
    pub vpc_region: Option<String>,
}

/// HostedZoneStatus defines the observed state of HostedZone
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostedZoneStatus {
    /// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
    /// that is used to contain resource sync state, account ownership,
    /// constructed ARN for the resource
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ackResourceMetadata")]
    pub ack_resource_metadata: Option<HostedZoneStatusAckResourceMetadata>,
    /// The value that you specified for CallerReference when you created the hosted
    /// zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "callerReference")]
    pub caller_reference: Option<String>,
    /// All CRs managed by ACK have a common `Status.Conditions` member that
    /// contains a collection of `ackv1alpha1.Condition` objects that describe
    /// the various terminal states of the CR and its backend AWS service API
    /// resource
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// A complex type that includes the Comment and PrivateZone elements. If you
    /// omitted the HostedZoneConfig and Comment elements from the request, the Config
    /// and Comment elements don't appear in the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<HostedZoneStatusConfig>,
    /// A complex type that describes the name servers for this hosted zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "delegationSet")]
    pub delegation_set: Option<HostedZoneStatusDelegationSet>,
    /// The ID that Amazon Route 53 assigned to the hosted zone when you created
    /// it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// If the hosted zone was created by another service, the service that created
    /// the hosted zone. When a hosted zone is created by another service, you can't
    /// edit or delete it using Route 53.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkedService")]
    pub linked_service: Option<HostedZoneStatusLinkedService>,
    /// The number of resource record sets in the hosted zone.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceRecordSetCount")]
    pub resource_record_set_count: Option<i64>,
}

/// All CRs managed by ACK have a common `Status.ACKResourceMetadata` member
/// that is used to contain resource sync state, account ownership,
/// constructed ARN for the resource
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostedZoneStatusAckResourceMetadata {
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

/// A complex type that includes the Comment and PrivateZone elements. If you
/// omitted the HostedZoneConfig and Comment elements from the request, the Config
/// and Comment elements don't appear in the response.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostedZoneStatusConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateZone")]
    pub private_zone: Option<bool>,
}

/// A complex type that describes the name servers for this hosted zone.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostedZoneStatusDelegationSet {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "callerReference")]
    pub caller_reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nameServers")]
    pub name_servers: Option<Vec<String>>,
}

/// If the hosted zone was created by another service, the service that created
/// the hosted zone. When a hosted zone is created by another service, you can't
/// edit or delete it using Route 53.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HostedZoneStatusLinkedService {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "servicePrincipal")]
    pub service_principal: Option<String>,
}

