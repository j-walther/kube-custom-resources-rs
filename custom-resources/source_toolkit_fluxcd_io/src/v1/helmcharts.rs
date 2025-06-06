// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/fluxcd/source-controller/source.toolkit.fluxcd.io/v1/helmcharts.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use std::collections::BTreeMap;
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// HelmChartSpec specifies the desired state of a Helm chart.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[kube(group = "source.toolkit.fluxcd.io", version = "v1", kind = "HelmChart", plural = "helmcharts")]
#[kube(namespaced)]
#[kube(status = "HelmChartStatus")]
#[kube(schema = "disabled")]
#[kube(derive="PartialEq")]
pub struct HelmChartSpec {
    /// Chart is the name or path the Helm chart is available at in the
    /// SourceRef.
    pub chart: String,
    /// IgnoreMissingValuesFiles controls whether to silently ignore missing values
    /// files rather than failing.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreMissingValuesFiles")]
    pub ignore_missing_values_files: Option<bool>,
    /// Interval at which the HelmChart SourceRef is checked for updates.
    /// This interval is approximate and may be subject to jitter to ensure
    /// efficient use of resources.
    pub interval: String,
    /// ReconcileStrategy determines what enables the creation of a new artifact.
    /// Valid values are ('ChartVersion', 'Revision').
    /// See the documentation of the values for an explanation on their behavior.
    /// Defaults to ChartVersion when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reconcileStrategy")]
    pub reconcile_strategy: Option<HelmChartReconcileStrategy>,
    /// SourceRef is the reference to the Source the chart is available at.
    #[serde(rename = "sourceRef")]
    pub source_ref: HelmChartSourceRef,
    /// Suspend tells the controller to suspend the reconciliation of this
    /// source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspend: Option<bool>,
    /// ValuesFiles is an alternative list of values files to use as the chart
    /// values (values.yaml is not included by default), expected to be a
    /// relative path in the SourceRef.
    /// Values files are merged in the order of this list with the last file
    /// overriding the first. Ignored when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valuesFiles")]
    pub values_files: Option<Vec<String>>,
    /// Verify contains the secret name containing the trusted public keys
    /// used to verify the signature and specifies which provider to use to check
    /// whether OCI image is authentic.
    /// This field is only supported when using HelmRepository source with spec.type 'oci'.
    /// Chart dependencies, which are not bundled in the umbrella chart artifact, are not verified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify: Option<HelmChartVerify>,
    /// Version is the chart version semver expression, ignored for charts from
    /// GitRepository and Bucket sources. Defaults to latest when omitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// HelmChartSpec specifies the desired state of a Helm chart.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HelmChartReconcileStrategy {
    ChartVersion,
    Revision,
}

/// SourceRef is the reference to the Source the chart is available at.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmChartSourceRef {
    /// APIVersion of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// Kind of the referent, valid values are ('HelmRepository', 'GitRepository',
    /// 'Bucket').
    pub kind: HelmChartSourceRefKind,
    /// Name of the referent.
    pub name: String,
}

/// SourceRef is the reference to the Source the chart is available at.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HelmChartSourceRefKind {
    HelmRepository,
    GitRepository,
    Bucket,
}

/// Verify contains the secret name containing the trusted public keys
/// used to verify the signature and specifies which provider to use to check
/// whether OCI image is authentic.
/// This field is only supported when using HelmRepository source with spec.type 'oci'.
/// Chart dependencies, which are not bundled in the umbrella chart artifact, are not verified.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct HelmChartVerify {
    /// MatchOIDCIdentity specifies the identity matching criteria to use
    /// while verifying an OCI artifact which was signed using Cosign keyless
    /// signing. The artifact's identity is deemed to be verified if any of the
    /// specified matchers match against the identity.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "matchOIDCIdentity")]
    pub match_oidc_identity: Option<Vec<HelmChartVerifyMatchOidcIdentity>>,
    /// Provider specifies the technology used to sign the OCI Artifact.
    pub provider: HelmChartVerifyProvider,
    /// SecretRef specifies the Kubernetes Secret containing the
    /// trusted public keys.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<HelmChartVerifySecretRef>,
}

/// OIDCIdentityMatch specifies options for verifying the certificate identity,
/// i.e. the issuer and the subject of the certificate.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmChartVerifyMatchOidcIdentity {
    /// Issuer specifies the regex pattern to match against to verify
    /// the OIDC issuer in the Fulcio certificate. The pattern must be a
    /// valid Go regular expression.
    pub issuer: String,
    /// Subject specifies the regex pattern to match against to verify
    /// the identity subject in the Fulcio certificate. The pattern must
    /// be a valid Go regular expression.
    pub subject: String,
}

/// Verify contains the secret name containing the trusted public keys
/// used to verify the signature and specifies which provider to use to check
/// whether OCI image is authentic.
/// This field is only supported when using HelmRepository source with spec.type 'oci'.
/// Chart dependencies, which are not bundled in the umbrella chart artifact, are not verified.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum HelmChartVerifyProvider {
    #[serde(rename = "cosign")]
    Cosign,
    #[serde(rename = "notation")]
    Notation,
}

/// SecretRef specifies the Kubernetes Secret containing the
/// trusted public keys.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmChartVerifySecretRef {
    /// Name of the referent.
    pub name: String,
}

/// HelmChartStatus records the observed state of the HelmChart.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmChartStatus {
    /// Artifact represents the output of the last successful reconciliation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifact: Option<HelmChartStatusArtifact>,
    /// Conditions holds the conditions for the HelmChart.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// LastHandledReconcileAt holds the value of the most recent
    /// reconcile request value, so a change of the annotation value
    /// can be detected.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lastHandledReconcileAt")]
    pub last_handled_reconcile_at: Option<String>,
    /// ObservedChartName is the last observed chart name as specified by the
    /// resolved chart reference.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedChartName")]
    pub observed_chart_name: Option<String>,
    /// ObservedGeneration is the last observed generation of the HelmChart
    /// object.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ObservedSourceArtifactRevision is the last observed Artifact.Revision
    /// of the HelmChartSpec.SourceRef.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedSourceArtifactRevision")]
    pub observed_source_artifact_revision: Option<String>,
    /// ObservedValuesFiles are the observed value files of the last successful
    /// reconciliation.
    /// It matches the chart in the last successfully reconciled artifact.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedValuesFiles")]
    pub observed_values_files: Option<Vec<String>>,
    /// URL is the dynamic fetch link for the latest Artifact.
    /// It is provided on a "best effort" basis, and using the precise
    /// BucketStatus.Artifact data is recommended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Artifact represents the output of the last successful reconciliation.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct HelmChartStatusArtifact {
    /// Digest is the digest of the file in the form of '<algorithm>:<checksum>'.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// LastUpdateTime is the timestamp corresponding to the last update of the
    /// Artifact.
    #[serde(rename = "lastUpdateTime")]
    pub last_update_time: String,
    /// Metadata holds upstream information such as OCI annotations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BTreeMap<String, String>>,
    /// Path is the relative file path of the Artifact. It can be used to locate
    /// the file in the root of the Artifact storage on the local file system of
    /// the controller managing the Source.
    pub path: String,
    /// Revision is a human-readable identifier traceable in the origin source
    /// system. It can be a Git commit SHA, Git tag, a Helm chart version, etc.
    pub revision: String,
    /// Size is the number of bytes in the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// URL is the HTTP address of the Artifact as exposed by the controller
    /// managing the Source. It can be used to retrieve the Artifact for
    /// consumption, e.g. by another controller applying the Artifact contents.
    pub url: String,
}

