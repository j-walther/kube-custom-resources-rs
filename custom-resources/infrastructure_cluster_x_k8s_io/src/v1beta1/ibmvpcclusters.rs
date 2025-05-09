// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kubernetes-sigs/cluster-api-provider-ibmcloud/infrastructure.cluster.x-k8s.io/v1beta1/ibmvpcclusters.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// IBMVPCClusterSpec defines the desired state of IBMVPCCluster.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "infrastructure.cluster.x-k8s.io", version = "v1beta1", kind = "IBMVPCCluster", plural = "ibmvpcclusters")]
#[kube(namespaced)]
#[kube(status = "IBMVPCClusterStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct IBMVPCClusterSpec {
    /// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneEndpoint")]
    pub control_plane_endpoint: Option<IBMVPCClusterControlPlaneEndpoint>,
    /// ControlPlaneLoadBalancer is optional configuration for customizing control plane behavior.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneLoadBalancer")]
    pub control_plane_load_balancer: Option<IBMVPCClusterControlPlaneLoadBalancer>,
    /// The IBM Cloud Region the cluster lives in.
    pub region: String,
    /// The VPC resources should be created under the resource group.
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    /// The Name of VPC.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,
    /// The Name of availability zone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zone: Option<String>,
}

/// ControlPlaneEndpoint represents the endpoint used to communicate with the control plane.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCClusterControlPlaneEndpoint {
    /// host is the hostname on which the API server is serving.
    pub host: String,
    /// port is the port on which the API server is serving.
    pub port: i32,
}

/// ControlPlaneLoadBalancer is optional configuration for customizing control plane behavior.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCClusterControlPlaneLoadBalancer {
    /// Name sets the name of the VPC load balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// IBMVPCClusterStatus defines the observed state of IBMVPCCluster.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCClusterStatus {
    /// Conditions defines current service state of the load balancer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ControlPlaneLoadBalancerState is the status of the load balancer.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "controlPlaneLoadBalancerState")]
    pub control_plane_load_balancer_state: Option<String>,
    /// Ready is true when the provider resource is ready.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ready: Option<bool>,
    /// Subnet describes a subnet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subnet: Option<IBMVPCClusterStatusSubnet>,
    /// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster
    /// Important: Run "make" to regenerate code after modifying this file
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vpc: Option<IBMVPCClusterStatusVpc>,
    /// VPCEndpoint describes a VPCEndpoint.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "vpcEndpoint")]
    pub vpc_endpoint: Option<IBMVPCClusterStatusVpcEndpoint>,
}

/// Subnet describes a subnet.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCClusterStatusSubnet {
    pub cidr: String,
    pub id: String,
    pub name: String,
    pub zone: String,
}

/// INSERT ADDITIONAL STATUS FIELD - define observed state of cluster
/// Important: Run "make" to regenerate code after modifying this file
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCClusterStatusVpc {
    pub id: String,
    pub name: String,
}

/// VPCEndpoint describes a VPCEndpoint.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct IBMVPCClusterStatusVpcEndpoint {
    pub address: String,
    /// Deprecated: This field has no function and is going to be removed in the next release.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "floatingIPID")]
    pub floating_ipid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerIPID")]
    pub load_balancer_ipid: Option<String>,
}

