// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/tigera/operator/operator.tigera.io/v1/managers.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// Specification of the desired state for the Calico Enterprise manager.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "operator.tigera.io", version = "v1", kind = "Manager", plural = "managers")]
#[kube(status = "ManagerStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ManagerSpec {
    /// ManagerDeployment configures the Manager Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "managerDeployment")]
    pub manager_deployment: Option<ManagerManagerDeployment>,
}

/// ManagerDeployment configures the Manager Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagerManagerDeployment {
    /// Spec is the specification of the Manager Deployment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ManagerManagerDeploymentSpec>,
}

/// Spec is the specification of the Manager Deployment.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagerManagerDeploymentSpec {
    /// Template describes the Manager Deployment pod that will be created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<ManagerManagerDeploymentSpecTemplate>,
}

/// Template describes the Manager Deployment pod that will be created.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagerManagerDeploymentSpecTemplate {
    /// Spec is the Manager Deployment's PodSpec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ManagerManagerDeploymentSpecTemplateSpec>,
}

/// Spec is the Manager Deployment's PodSpec.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagerManagerDeploymentSpecTemplateSpec {
    /// Containers is a list of Manager containers.
    /// If specified, this overrides the specified Manager Deployment containers.
    /// If omitted, the Manager Deployment will use its default values for its containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ManagerManagerDeploymentSpecTemplateSpecContainers>>,
    /// InitContainers is a list of Manager init containers.
    /// If specified, this overrides the specified Manager Deployment init containers.
    /// If omitted, the Manager Deployment will use its default values for its init containers.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "initContainers")]
    pub init_containers: Option<Vec<ManagerManagerDeploymentSpecTemplateSpecInitContainers>>,
}

/// ManagerDeploymentContainer is a Manager Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagerManagerDeploymentSpecTemplateSpecContainers {
    /// Name is an enum which identifies the Manager Deployment container by name.
    /// Supported values are: tigera-voltron, tigera-manager, tigera-ui-apis, and tigera-es-proxy (deprecated).
    pub name: ManagerManagerDeploymentSpecTemplateSpecContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named Manager Deployment container's resources.
    /// If omitted, the Manager Deployment will use its default value for this container's resources.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ManagerManagerDeploymentSpecTemplateSpecContainersResources>,
}

/// ManagerDeploymentContainer is a Manager Deployment container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManagerManagerDeploymentSpecTemplateSpecContainersName {
    #[serde(rename = "tigera-voltron")]
    TigeraVoltron,
    #[serde(rename = "tigera-manager")]
    TigeraManager,
    #[serde(rename = "tigera-es-proxy")]
    TigeraEsProxy,
    #[serde(rename = "tigera-ui-apis")]
    TigeraUiApis,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named Manager Deployment container's resources.
/// If omitted, the Manager Deployment will use its default value for this container's resources.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagerManagerDeploymentSpecTemplateSpecContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<ManagerManagerDeploymentSpecTemplateSpecContainersResourcesClaims>>,
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
pub struct ManagerManagerDeploymentSpecTemplateSpecContainersResourcesClaims {
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

/// ManagerDeploymentInitContainer is a Manager Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ManagerManagerDeploymentSpecTemplateSpecInitContainers {
    /// Name is an enum which identifies the Manager Deployment init container by name.
    /// Supported values are: manager-tls-key-cert-provisioner, internal-manager-tls-key-cert-provisioner, tigera-voltron-linseed-tls-key-cert-provisioner
    pub name: ManagerManagerDeploymentSpecTemplateSpecInitContainersName,
    /// Resources allows customization of limits and requests for compute resources such as cpu and memory.
    /// If specified, this overrides the named Manager Deployment init container's resources.
    /// If omitted, the Manager Deployment will use its default value for this init container's resources.
    /// If used in conjunction with the deprecated ComponentResources, then this value takes precedence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<ManagerManagerDeploymentSpecTemplateSpecInitContainersResources>,
}

/// ManagerDeploymentInitContainer is a Manager Deployment init container.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ManagerManagerDeploymentSpecTemplateSpecInitContainersName {
    #[serde(rename = "manager-tls-key-cert-provisioner")]
    ManagerTlsKeyCertProvisioner,
    #[serde(rename = "internal-manager-tls-key-cert-provisioner")]
    InternalManagerTlsKeyCertProvisioner,
    #[serde(rename = "tigera-voltron-linseed-tls-key-cert-provisioner")]
    TigeraVoltronLinseedTlsKeyCertProvisioner,
}

/// Resources allows customization of limits and requests for compute resources such as cpu and memory.
/// If specified, this overrides the named Manager Deployment init container's resources.
/// If omitted, the Manager Deployment will use its default value for this init container's resources.
/// If used in conjunction with the deprecated ComponentResources, then this value takes precedence.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagerManagerDeploymentSpecTemplateSpecInitContainersResources {
    /// Claims lists the names of resources, defined in spec.resourceClaims,
    /// that are used by this container.
    /// This is an alpha field and requires enabling the
    /// DynamicResourceAllocation feature gate.
    /// This field is immutable. It can only be set for containers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<ManagerManagerDeploymentSpecTemplateSpecInitContainersResourcesClaims>>,
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
pub struct ManagerManagerDeploymentSpecTemplateSpecInitContainersResourcesClaims {
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

/// Most recently observed state for the Calico Enterprise manager.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ManagerStatus {
    /// Conditions represents the latest observed set of conditions for the component. A component may be one or more of
    /// Ready, Progressing, Degraded or other customer types.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// State provides user-readable status.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

