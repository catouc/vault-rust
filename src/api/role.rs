use crate::api::endpoint::Endpoint;
use std::borrow::Cow;
use std::collections::HashMap;


pub struct EndpointRole {
    pub auth_provider_name: String,
    pub role_name: String,
}

impl EndpointRole {
    pub fn new(auth_provider_name: String, role_name: String) -> EndpointRole {
        EndpointRole{auth_provider_name, role_name}
    }
}

impl Endpoint for EndpointRole {
    fn endpoint(&self) -> Cow<'static, str> {
        Cow::from(format!("v1/auth/{}/role/{}", self.auth_provider_name, self.role_name))
    }
}

pub struct RoleResponse {

    pub data: Role,
}

pub struct Role {
    pub bound_audiences: Vec<String>,
    pub bound_claims: HashMap<String, Vec<String>>,
    pub user_claim: Option<String>,
    pub token_policies: Vec<String>,
}

