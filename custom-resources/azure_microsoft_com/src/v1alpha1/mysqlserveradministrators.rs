// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/Azure/azure-service-operator/azure.microsoft.com/v1alpha1/mysqlserveradministrators.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "azure.microsoft.com", version = "v1alpha1", kind = "MySQLServerAdministrator", plural = "mysqlserveradministrators")]
#[kube(namespaced)]
#[kube(status = "MySQLServerAdministratorStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct MySQLServerAdministratorSpec {
    /// AdministratorType: The type of administrator.
    #[serde(rename = "administratorType")]
    pub administrator_type: MySQLServerAdministratorAdministratorType,
    /// Login: The server administrator login account name. For example: "myuser@microsoft.com" might be the login if specifying an AAD user. "my-mi" might be the name of a managed identity
    pub login: String,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    pub server: String,
    /// Sid: The server administrator Sid (Secure ID). If creating for an AAD user or group, this is the OID of the entity in AAD. For a managed identity this should be the Client ID (or app id) of the identity.
    pub sid: String,
    /// TenantId: The server Active Directory Administrator tenant id.
    #[serde(rename = "tenantId")]
    pub tenant_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MySQLServerAdministratorAdministratorType {
    ActiveDirectory,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct MySQLServerAdministratorStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "containsUpdate")]
    pub contains_update: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "failedProvisioning")]
    pub failed_provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "flattenedSecrets")]
    pub flattened_secrets: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrl")]
    pub polling_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollingUrlKind")]
    pub polling_url_kind: Option<MySQLServerAdministratorStatusPollingUrlKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceId")]
    pub resource_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "specHash")]
    pub spec_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// ASOStatus (AzureServiceOperatorsStatus) defines the observed state of resource actions
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MySQLServerAdministratorStatusPollingUrlKind {
    CreateOrUpdate,
    Delete,
}

