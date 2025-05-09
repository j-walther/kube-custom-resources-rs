// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/atlasmap/atlasmap-operator/atlasmap.io/v1alpha1/atlasmaps.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
}
use self::prelude::*;

/// AtlasMapSpec defines the desired state of AtlasMap
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "atlasmap.io", version = "v1alpha1", kind = "AtlasMap", plural = "atlasmaps")]
#[kube(namespaced)]
#[kube(status = "AtlasMapStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct AtlasMapSpec {
    /// The amount of CPU to limit
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limitCPU")]
    pub limit_cpu: Option<String>,
    /// The amount of memory to request
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limitMemory")]
    pub limit_memory: Option<String>,
    /// Replicas determines the desired number of running AtlasMap pods
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i32>,
    /// The amount of CPU to request
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestCPU")]
    pub request_cpu: Option<String>,
    /// The amount of memory to request
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestMemory")]
    pub request_memory: Option<String>,
    /// RouteHostName sets the host name to use on the Ingress or OpenShift Route
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "routeHostName")]
    pub route_host_name: Option<String>,
    /// Version sets the version of the container image used for AtlasMap
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// AtlasMapStatus defines the observed state of AtlasMap
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct AtlasMapStatus {
    /// The URL where AtlasMap can be accessed
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "URL")]
    pub url: Option<String>,
    /// The container image that AtlasMap is using
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The current phase that the AtlasMap resource is in
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
}

