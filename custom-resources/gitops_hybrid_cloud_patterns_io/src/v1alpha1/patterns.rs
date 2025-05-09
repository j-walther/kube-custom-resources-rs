// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/validatedpatterns/patterns-operator/gitops.hybrid-cloud-patterns.io/v1alpha1/patterns.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// PatternSpec defines the desired state of Pattern
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "gitops.hybrid-cloud-patterns.io", version = "v1alpha1", kind = "Pattern", plural = "patterns")]
#[kube(namespaced)]
#[kube(status = "PatternStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct PatternSpec {
    /// Analytics UUID. Leave empty to autogenerate a random one. Not PII information
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "analyticsUUID")]
    pub analytics_uuid: Option<String>,
    #[serde(rename = "clusterGroupName")]
    pub cluster_group_name: String,
    /// Comma separated capabilities to enable certain experimental features
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "experimentalCapabilities")]
    pub experimental_capabilities: Option<String>,
    /// .Name is dot separated per the helm --set syntax, such as:
    ///   global.something.field
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraParameters")]
    pub extra_parameters: Option<Vec<PatternExtraParameters>>,
    /// URLs to additional Helm parameter files
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extraValueFiles")]
    pub extra_value_files: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gitOpsSpec")]
    pub git_ops_spec: Option<PatternGitOpsSpec>,
    #[serde(rename = "gitSpec")]
    pub git_spec: PatternGitSpec,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "multiSourceConfig")]
    pub multi_source_config: Option<PatternMultiSourceConfig>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PatternExtraParameters {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PatternGitOpsSpec {
    /// Require manual intervention before Argo will sync new content. Default: False
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "manualSync")]
    pub manual_sync: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PatternGitSpec {
    /// Optional. FQDN of the git server if automatic parsing from TargetRepo is broken
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// (EXPERIMENTAL) Enable in-cluster git server (avoids the need of forking the upstream repository)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "inClusterGitServer")]
    pub in_cluster_git_server: Option<bool>,
    /// Upstream git repo containing the pattern to deploy. Used when in-cluster fork to point to the upstream pattern repository.
    /// Takes precedence over TargetRepo
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "originRepo")]
    pub origin_repo: Option<String>,
    /// (DEPRECATED) Branch, tag or commit in the upstream git repository. Does not support short-sha's. Default to HEAD
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "originRevision")]
    pub origin_revision: Option<String>,
    /// Interval in seconds to poll for drifts between origin and target repositories. Default: 180 seconds
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "pollInterval")]
    pub poll_interval: Option<i64>,
    /// Git repo containing the pattern to deploy. Must use https/http or, for ssh, git@server:foo/bar.git
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRepo")]
    pub target_repo: Option<String>,
    /// Branch, tag, or commit to deploy.  Does not support short-sha's. Default: HEAD
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "targetRevision")]
    pub target_revision: Option<String>,
    /// Optional. K8s secret name where the info for connecting to git can be found. The supported secrets are modeled after the
    /// private repositories in argo (https://argo-cd.readthedocs.io/en/stable/operator-manual/declarative-setup/#repositories)
    /// currently ssh and username+password are supported
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenSecret")]
    pub token_secret: Option<String>,
    /// Optional. K8s secret namespace where the token for connecting to git can be found
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "tokenSecretNamespace")]
    pub token_secret_namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PatternMultiSourceConfig {
    /// The git reference when deploying the clustergroup helm chart directly from a git repo
    /// Defaults to 'main'. (Only used when developing the clustergroup helm chart)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterGroupChartGitRevision")]
    pub cluster_group_chart_git_revision: Option<String>,
    /// Which chart version for the clustergroup helm chart. Defaults to "0.8.*"
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterGroupChartVersion")]
    pub cluster_group_chart_version: Option<String>,
    /// The url when deploying the clustergroup helm chart directly from a git repo
    /// Defaults to '' which means not used (Only used when developing the clustergroup helm chart)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterGroupGitRepoUrl")]
    pub cluster_group_git_repo_url: Option<String>,
    /// (EXPERIMENTAL) Enable multi-source support when deploying the clustergroup argo application
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The helm chart url to fetch the helm charts from in order to deploy the pattern. Defaults to https://charts.validatedpatterns.io/
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "helmRepoUrl")]
    pub helm_repo_url: Option<String>,
}

/// PatternStatus defines the observed state of Pattern
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PatternStatus {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "analyticsSent")]
    pub analytics_sent: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "analyticsUUID")]
    pub analytics_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appClusterDomain")]
    pub app_cluster_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<PatternStatusApplications>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterDomain")]
    pub cluster_domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterID")]
    pub cluster_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterName")]
    pub cluster_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterPlatform")]
    pub cluster_platform: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clusterVersion")]
    pub cluster_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<PatternStatusConditions>>,
    /// Last error encountered by the pattern
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastError")]
    pub last_error: Option<String>,
    /// Last action related to the pattern
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastStep")]
    pub last_step: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Number of updates to the pattern
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// PatternApplicationInfo defines the Applications
/// Status for the Pattern.
/// This structure is part of the PatternStatus as an array
/// The Application Status will be included as part of the Observed state of Pattern
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PatternStatusApplications {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthMessage")]
    pub health_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "healthStatus")]
    pub health_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "syncStatus")]
    pub sync_status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct PatternStatusConditions {
    /// Last time the condition transitioned from one status to another.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastTransitionTime")]
    pub last_transition_time: Option<String>,
    /// The last time this condition was updated.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// A human readable message indicating details about the transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Status of the condition, one of True, False, Unknown.
    pub status: String,
    /// Type of deployment condition.
    #[serde(rename = "type")]
    pub r#type: String,
}

