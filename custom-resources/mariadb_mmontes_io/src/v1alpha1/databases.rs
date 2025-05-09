// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/mariadb-operator/mariadb-operator/mariadb.mmontes.io/v1alpha1/databases.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
}
use self::prelude::*;

/// DatabaseSpec defines the desired state of Database
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "mariadb.mmontes.io", version = "v1alpha1", kind = "Database", plural = "databases")]
#[kube(namespaced)]
#[kube(status = "DatabaseStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct DatabaseSpec {
    /// CharacterSet to use in the Database.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "characterSet")]
    pub character_set: Option<String>,
    /// CharacterSet to use in the Database.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collate: Option<String>,
    /// MariaDBRef is a reference to a MariaDB object.
    #[serde(rename = "mariaDbRef")]
    pub maria_db_ref: DatabaseMariaDbRef,
    /// Name overrides the default Database name provided by metadata.name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// RequeueInterval is used to perform requeue reconcilizations.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requeueInterval")]
    pub requeue_interval: Option<String>,
    /// RetryInterval is the interval used to perform retries.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryInterval")]
    pub retry_interval: Option<String>,
}

/// MariaDBRef is a reference to a MariaDB object.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseMariaDbRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2]. For example, if the object reference is to a container within a pod, this would take on a value like: "spec.containers{name}" (where "name" refers to the name of the container that triggered the event) or if no container name is specified "spec.containers[2]" (container with index 2 in this pod). This syntax is chosen only to have some well-defined way of referencing a part of an object. TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// WaitForIt indicates whether the controller using this reference should wait for MariaDB to be ready.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "waitForIt")]
    pub wait_for_it: Option<bool>,
}

/// DatabaseStatus defines the observed state of Database
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct DatabaseStatus {
    /// Conditions for the Database object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
}

