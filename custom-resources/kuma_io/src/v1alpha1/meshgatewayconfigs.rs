// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/kumahq/kuma/kuma.io/v1alpha1/meshgatewayconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
}
use self::prelude::*;

/// MeshGatewayConfigSpec specifies the options available for a Kuma MeshGateway.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "kuma.io", version = "v1alpha1", kind = "MeshGatewayConfig", plural = "meshgatewayconfigs")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct MeshGatewayConfigSpec {
    /// CrossMesh specifies whether listeners configured by this gateway are
    /// cross mesh listeners.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "crossMesh")]
    pub cross_mesh: Option<bool>,
    /// PodTemplate configures the Pod owned by this config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "podTemplate")]
    pub pod_template: Option<MeshGatewayConfigPodTemplate>,
    /// Replicas is the number of dataplane proxy replicas to create. For
    /// now this is a fixed number, but in the future it could be
    /// automatically scaled based on metrics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// Resources specifies the compute resources for the proxy container.
    /// The default can be set in the control plane config.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<MeshGatewayConfigResources>,
    /// ServiceTemplate configures the Service owned by this config.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceTemplate")]
    pub service_template: Option<MeshGatewayConfigServiceTemplate>,
    /// ServiceType specifies the type of managed Service that will be
    /// created to expose the dataplane proxies to traffic from outside
    /// the cluster. The ports to expose will be taken from the matching Gateway
    /// resource. If there is no matching Gateway, the managed Service will
    /// be deleted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceType")]
    pub service_type: Option<MeshGatewayConfigServiceType>,
    /// Tags specifies a set of Kuma tags that are included in the
    /// MeshGatewayInstance and thus propagated to every Dataplane generated to
    /// serve the MeshGateway.
    /// These tags should include a maximum of one `kuma.io/service` tag.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<BTreeMap<String, String>>,
}

/// PodTemplate configures the Pod owned by this config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigPodTemplate {
    /// Metadata holds metadata configuration for a Service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MeshGatewayConfigPodTemplateMetadata>,
    /// Spec holds some customizable fields of a Pod.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<MeshGatewayConfigPodTemplateSpec>,
}

/// Metadata holds metadata configuration for a Service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigPodTemplateMetadata {
    /// Annotations holds annotations to be set on an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels holds labels to be set on an objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// Spec holds some customizable fields of a Pod.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigPodTemplateSpec {
    /// Container corresponds to PodSpec.Container
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container: Option<MeshGatewayConfigPodTemplateSpecContainer>,
    /// PodSecurityContext corresponds to PodSpec.SecurityContext
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityContext")]
    pub security_context: Option<MeshGatewayConfigPodTemplateSpecSecurityContext>,
    /// ServiceAccountName corresponds to PodSpec.ServiceAccountName.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serviceAccountName")]
    pub service_account_name: Option<String>,
}

/// Container corresponds to PodSpec.Container
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigPodTemplateSpecContainer {
    /// ContainerSecurityContext corresponds to PodSpec.Container.SecurityContext
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "securityContext")]
    pub security_context: Option<MeshGatewayConfigPodTemplateSpecContainerSecurityContext>,
}

/// ContainerSecurityContext corresponds to PodSpec.Container.SecurityContext
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigPodTemplateSpecContainerSecurityContext {
    /// ReadOnlyRootFilesystem corresponds to PodSpec.Container.SecurityContext.ReadOnlyRootFilesystem
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readOnlyRootFilesystem")]
    pub read_only_root_filesystem: Option<bool>,
}

/// PodSecurityContext corresponds to PodSpec.SecurityContext
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigPodTemplateSpecSecurityContext {
    /// FSGroup corresponds to PodSpec.SecurityContext.FSGroup
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fsGroup")]
    pub fs_group: Option<i64>,
}

/// Resources specifies the compute resources for the proxy container.
/// The default can be set in the control plane config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// 
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// 
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<MeshGatewayConfigResourcesClaims>>,
    /// Limits describes the maximum amount of compute resources allowed.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limits: Option<BTreeMap<String, IntOrString>>,
    /// Requests describes the minimum amount of compute resources required.
    /// If Requests is omitted for a container, it defaults to Limits if that is explicitly specified,
    /// otherwise to an implementation-defined value. Requests cannot exceed Limits.
    /// More info: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<BTreeMap<String, IntOrString>>,
}

/// ResourceClaim references one entry in PodSpec.ResourceClaims.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigResourcesClaims {
    /// Name must match the name of one entry in pod.spec.resourceClaims of
    /// the Pod where this field is used. It makes that resource available
    /// inside a container.
    pub name: String,
    /// Request is the name chosen for a request in the referenced claim.
    /// If empty, everything from the claim is made available, otherwise
    /// only the result of this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
}

/// ServiceTemplate configures the Service owned by this config.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigServiceTemplate {
    /// Metadata holds metadata configuration for a Service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MeshGatewayConfigServiceTemplateMetadata>,
    /// Spec holds some customizable fields of a Service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<MeshGatewayConfigServiceTemplateSpec>,
}

/// Metadata holds metadata configuration for a Service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigServiceTemplateMetadata {
    /// Annotations holds annotations to be set on an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, String>>,
    /// Labels holds labels to be set on an objects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, String>>,
}

/// Spec holds some customizable fields of a Service.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigServiceTemplateSpec {
    /// LoadBalancerIP corresponds to ServiceSpec.LoadBalancerIP.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "loadBalancerIP")]
    pub load_balancer_ip: Option<String>,
}

/// MeshGatewayConfigSpec specifies the options available for a Kuma MeshGateway.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MeshGatewayConfigServiceType {
    LoadBalancer,
    #[serde(rename = "ClusterIP")]
    ClusterIp,
    NodePort,
}

/// MeshGatewayConfigStatus holds information about the status of the gateway
/// instance.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MeshGatewayConfigStatus {
}

