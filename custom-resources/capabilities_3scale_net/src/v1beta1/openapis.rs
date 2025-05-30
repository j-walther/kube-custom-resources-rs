// WARNING: generated by kopium - manual changes will be overwritten
// kopium command: kopium --docs --derive=Default --derive=PartialEq --smart-derive-elision --filename crd-catalog/3scale/3scale-operator/capabilities.3scale.net/v1beta1/openapis.yaml
// kopium version: 0.21.2

#[allow(unused_imports)]
mod prelude {
    pub use kube::CustomResource;
    pub use serde::{Serialize, Deserialize};
    pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
    pub use k8s_openapi::api::core::v1::ObjectReference;
}
use self::prelude::*;

/// OpenAPISpec defines the desired state of OpenAPI
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[kube(group = "capabilities.3scale.net", version = "v1beta1", kind = "OpenAPI", plural = "openapis")]
#[kube(namespaced)]
#[kube(status = "OpenAPIStatus")]
#[kube(schema = "disabled")]
#[kube(derive="Default")]
#[kube(derive="PartialEq")]
pub struct OpenAPISpec {
    /// OIDCSpec defines the desired configuration of OpenID Connect Authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oidc: Option<OpenAPIOidc>,
    /// OpenAPIRef Reference to the OpenAPI Specification
    #[serde(rename = "openapiRef")]
    pub openapi_ref: OpenAPIOpenapiRef,
    /// PrefixMatching Use prefix matching instead of strict matching on mapping rules derived from openapi operations
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prefixMatching")]
    pub prefix_matching: Option<bool>,
    /// PrivateAPIHostHeader Custom host header sent by the API gateway to the private API
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateAPIHostHeader")]
    pub private_api_host_header: Option<String>,
    /// PrivateAPISecretToken Custom secret token sent by the API gateway to the private API
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateAPISecretToken")]
    pub private_api_secret_token: Option<String>,
    /// PrivateBaseURL Custom private base URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "privateBaseURL")]
    pub private_base_url: Option<String>,
    /// ProductSystemName 3scale product system name
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "productSystemName")]
    pub product_system_name: Option<String>,
    /// ProductionPublicBaseURL Custom public production URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "productionPublicBaseURL")]
    pub production_public_base_url: Option<String>,
    /// ProviderAccountRef references account provider credentials
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountRef")]
    pub provider_account_ref: Option<OpenAPIProviderAccountRef>,
    /// StagingPublicBaseURL Custom public staging URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stagingPublicBaseURL")]
    pub staging_public_base_url: Option<String>,
}

/// OIDCSpec defines the desired configuration of OpenID Connect Authentication
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OpenAPIOidc {
    /// AuthenticationFlow specifies OAuth2.0 authorization grant type
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "authenticationFlow")]
    pub authentication_flow: Option<OpenAPIOidcAuthenticationFlow>,
    /// Credentials Location available options:
    /// headers: As HTTP Headers
    /// query: As query parameters (GET) or body parameters (POST/PUT/DELETE)
    /// authorization: As HTTP Basic Authentication
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credentials: Option<OpenAPIOidcCredentials>,
    /// GatewayResponseSpec defines the desired gateway response configuration
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gatewayResponse")]
    pub gateway_response: Option<OpenAPIOidcGatewayResponse>,
    /// Issuer is the OIDC issuer
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "issuerEndpoint")]
    pub issuer_endpoint: Option<String>,
    /// IssuerEndpointRef  is the reference to OIDC issuer Secret that contains IssuerEndpoint
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "issuerEndpointRef")]
    pub issuer_endpoint_ref: Option<OpenAPIOidcIssuerEndpointRef>,
    /// IssuerType is the type of the OIDC issuer
    #[serde(rename = "issuerType")]
    pub issuer_type: OpenAPIOidcIssuerType,
    /// JwtClaimWithClientID is the JSON Web Token (JWT) Claim with ClientID that contains the clientID. Defaults to 'azp'.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtClaimWithClientID")]
    pub jwt_claim_with_client_id: Option<String>,
    /// JwtClaimWithClientIDType sets to process the ClientID Token Claim value as a string or as a liquid template.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "jwtClaimWithClientIDType")]
    pub jwt_claim_with_client_id_type: Option<OpenAPIOidcJwtClaimWithClientIdType>,
    /// SecuritySpec defines the desired state of Authentication Security
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub security: Option<OpenAPIOidcSecurity>,
}

/// AuthenticationFlow specifies OAuth2.0 authorization grant type
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIOidcAuthenticationFlow {
    #[serde(rename = "directAccessGrantsEnabled")]
    pub direct_access_grants_enabled: bool,
    #[serde(rename = "implicitFlowEnabled")]
    pub implicit_flow_enabled: bool,
    #[serde(rename = "serviceAccountsEnabled")]
    pub service_accounts_enabled: bool,
    /// OIDCIssuer is the OIDC issuer
    #[serde(rename = "standardFlowEnabled")]
    pub standard_flow_enabled: bool,
}

/// OIDCSpec defines the desired configuration of OpenID Connect Authentication
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OpenAPIOidcCredentials {
    #[serde(rename = "headers")]
    Headers,
    #[serde(rename = "query")]
    Query,
    #[serde(rename = "authorization")]
    Authorization,
}

/// GatewayResponseSpec defines the desired gateway response configuration
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIOidcGatewayResponse {
    /// ErrorAuthFailed specifies the response body when authentication fails
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorAuthFailed")]
    pub error_auth_failed: Option<String>,
    /// ErrorAuthMissing specifies the response body when authentication is missing
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorAuthMissing")]
    pub error_auth_missing: Option<String>,
    /// ErrorHeadersAuthFailed specifies the Content-Type header when authentication fails
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorHeadersAuthFailed")]
    pub error_headers_auth_failed: Option<String>,
    /// ErrorHeadersAuthMissing specifies the Content-Type header when authentication is missing
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorHeadersAuthMissing")]
    pub error_headers_auth_missing: Option<String>,
    /// ErrorHeadersLimitsExceeded specifies the Content-Type header when usage limit exceeded
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorHeadersLimitsExceeded")]
    pub error_headers_limits_exceeded: Option<String>,
    /// ErrorHeadersNoMatch specifies the Content-Type header when no match error
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorHeadersNoMatch")]
    pub error_headers_no_match: Option<String>,
    /// ErrorLimitsExceeded specifies the response body when usage limit exceeded
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorLimitsExceeded")]
    pub error_limits_exceeded: Option<String>,
    /// ErrorNoMatch specifies the response body when no match error
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorNoMatch")]
    pub error_no_match: Option<String>,
    /// ErrorStatusAuthFailed specifies the response code when authentication fails
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorStatusAuthFailed")]
    pub error_status_auth_failed: Option<i32>,
    /// ErrorStatusAuthMissing specifies the response code when authentication is missing
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorStatusAuthMissing")]
    pub error_status_auth_missing: Option<i32>,
    /// ErrorStatusLimitsExceeded specifies the response code when usage limit exceeded
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorStatusLimitsExceeded")]
    pub error_status_limits_exceeded: Option<i32>,
    /// ErrorStatusNoMatch specifies the response code when no match error
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "errorStatusNoMatch")]
    pub error_status_no_match: Option<i32>,
}

/// IssuerEndpointRef  is the reference to OIDC issuer Secret that contains IssuerEndpoint
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIOidcIssuerEndpointRef {
    /// name is unique within a namespace to reference a secret resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// namespace defines the space within which the secret name must be unique.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// OIDCSpec defines the desired configuration of OpenID Connect Authentication
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OpenAPIOidcIssuerType {
    #[serde(rename = "keycloak")]
    Keycloak,
    #[serde(rename = "rest")]
    Rest,
}

/// OIDCSpec defines the desired configuration of OpenID Connect Authentication
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum OpenAPIOidcJwtClaimWithClientIdType {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "liquid")]
    Liquid,
}

/// SecuritySpec defines the desired state of Authentication Security
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIOidcSecurity {
    /// HostHeader Lets you define a custom Host request header. This is needed if your API backend only accepts traffic from a specific host.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hostHeader")]
    pub host_header: Option<String>,
    /// SecretToken Enables you to block any direct developer requests to your API backend;
    /// each 3scale API gateway call to your API backend contains a request header called X-3scale-proxy-secret-token.
    /// The value of this header can be set by you here. It's up to you ensure your backend only allows calls with this secret header.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretToken")]
    pub secret_token: Option<String>,
}

/// OpenAPIRef Reference to the OpenAPI Specification
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIOpenapiRef {
    /// SecretRef refers to the secret object that contains the OpenAPI Document
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "secretRef")]
    pub secret_ref: Option<ObjectReference>,
    /// URL Remote URL from where to fetch the OpenAPI Document
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// SecretRef refers to the secret object that contains the OpenAPI Document
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIOpenapiRefSecretRef {
    /// API version of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apiVersion")]
    pub api_version: Option<String>,
    /// If referring to a piece of an object instead of an entire object, this string
    /// should contain a valid JSON/Go field access statement, such as desiredState.manifest.containers[2].
    /// For example, if the object reference is to a container within a pod, this would take on a value like:
    /// "spec.containers{name}" (where "name" refers to the name of the container that triggered
    /// the event) or if no container name is specified "spec.containers[2]" (container with
    /// index 2 in this pod). This syntax is chosen only to have some well-defined way of
    /// referencing a part of an object.
    /// TODO: this design is not final and this field is subject to change in the future.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fieldPath")]
    pub field_path: Option<String>,
    /// Kind of the referent.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Namespace of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/namespaces/
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// Specific resourceVersion to which this reference is made, if any.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "resourceVersion")]
    pub resource_version: Option<String>,
    /// UID of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#uids
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

/// ProviderAccountRef references account provider credentials
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIProviderAccountRef {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// OpenAPIStatus defines the observed state of OpenAPI
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIStatus {
    /// BackendResourceNames contains a list of references to the managed 3scale backends
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendResourceNames")]
    pub backend_resource_names: Option<Vec<OpenAPIStatusBackendResourceNames>>,
    /// Current state of the openapi resource.
    /// Conditions represent the latest available observations of an object's state
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// ObservedGeneration reflects the generation of the most recently observed Backend Spec.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "observedGeneration")]
    pub observed_generation: Option<i64>,
    /// ProductResourceName references the managed 3scale product
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "productResourceName")]
    pub product_resource_name: Option<OpenAPIStatusProductResourceName>,
    /// ProviderAccountHost contains the 3scale account's provider URL
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "providerAccountHost")]
    pub provider_account_host: Option<String>,
}

/// LocalObjectReference contains enough information to let you locate the
/// referenced object inside the same namespace.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIStatusBackendResourceNames {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// ProductResourceName references the managed 3scale product
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct OpenAPIStatusProductResourceName {
    /// Name of the referent.
    /// More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    /// TODO: Add other useful fields. apiVersion, kind, uid?
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

