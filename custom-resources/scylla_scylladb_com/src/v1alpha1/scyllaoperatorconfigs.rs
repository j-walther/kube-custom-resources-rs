// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/scylladb/scylla-operator/scylla.scylladb.com/v1alpha1/scyllaoperatorconfigs.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// spec defines the desired state of the operator.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "scylla.scylladb.com", version = "v1alpha1", kind = "ScyllaOperatorConfig", plural = "scyllaoperatorconfigs")]
#[kube(status = "ScyllaOperatorConfigStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct ScyllaOperatorConfigSpec {
    /// configuredClusterDomain allows users to set the configured Kubernetes cluster domain explicitly, instead of letting Scylla Operator automatically discover it.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configuredClusterDomain")]
    pub configured_cluster_domain: Option<String>,
    /// scyllaUtilsImage is a ScyllaDB image used for running ScyllaDB utilities.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scyllaUtilsImage")]
    pub scylla_utils_image: Option<String>,
    /// unsupportedBashToolsImageOverride allows to adjust a generic Bash image with extra tools used by the operator
    /// for auxiliary purposes.
    /// Setting this field renders your cluster unsupported. Use at your own risk.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unsupportedBashToolsImageOverride")]
    pub unsupported_bash_tools_image_override: Option<String>,
    /// unsupportedGrafanaImageOverride allows to adjust Grafana image used by the operator
    /// for testing, dev or emergencies.
    /// Setting this field renders your cluster unsupported. Use at your own risk.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unsupportedGrafanaImageOverride")]
    pub unsupported_grafana_image_override: Option<String>,
    /// unsupportedPrometheusVersionOverride allows to adjust Prometheus version used by the operator
    /// for testing, dev or emergencies.
    /// Setting this field renders your cluster unsupported. Use at your own risk.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "unsupportedPrometheusVersionOverride")]
    pub unsupported_prometheus_version_override: Option<String>,
}

/// status defines the observed state of the operator.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ScyllaOperatorConfigStatus {
    /// bashToolsImage is a generic Bash image with extra tools used by the operator for auxiliary purposes.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bashToolsImage")]
    pub bash_tools_image: Option<String>,
    /// clusterDomain is the Kubernetes cluster domain used by the Scylla Operator.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterDomain")]
    pub cluster_domain: Option<String>,
    /// conditions hold conditions describing ScyllaOperatorConfig state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// grafanaImage is the image used by the operator to create a Grafana instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "grafanaImage")]
    pub grafana_image: Option<String>,
    /// observedGeneration is the most recent generation observed for this ScyllaOperatorConfig. It corresponds to the
    /// ScyllaOperatorConfig's generation, which is updated on mutation by the API Server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// prometheusVersion is the Prometheus version used by the operator to create a Prometheus instance.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prometheusVersion")]
    pub prometheus_version: Option<String>,
    /// scyllaDBUtilsImage is the ScyllaDB image used for running ScyllaDB utilities.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "scyllaDBUtilsImage")]
    pub scylla_db_utils_image: Option<String>,
}

