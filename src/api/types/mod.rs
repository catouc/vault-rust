use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseMetadata {
    pub request_id: String,
    pub lease_id: String,
    pub renewable: bool,
    pub lease_duration: u32,
    // TODO:
    // * wrap_info
    // * warnings
    // * auth
}

#[derive(Deserialize, Debug)]
pub struct JWTOIDCConfigResponse {
    #[serde(flatten)]
    pub response_metadata: ResponseMetadata,
    pub data: JWTOIDCConfig,
}

#[derive(Deserialize, Debug)]
pub struct JWTOIDCConfig {
    bound_issuer: Option<String>,
    default_role: String,
    jwks_ca_pem: Option<String>,
    jwks_url: Option<String>,
    jwt_supported_algs: Vec<String>,
    jwt_validation_pubkeys: Option<Vec<String>>,
    namespace_in_state: bool,
    oidc_client_id: Option<String>,
    oidc_client_secret: Option<String>,
    oidc_discovery_ca_pem: Option<String>,
    oidc_discovery_url: String,
    oidc_response_mode: Option<String>,
    oidc_response_types: Option<String>,
    provider_config: HashMap<String, String>,
}

