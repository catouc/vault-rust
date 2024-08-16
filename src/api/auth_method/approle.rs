use crate::{ auth_roles_endpoint, auth_role_endpoint};
use crate::api::endpoint::Endpoint;
use std::borrow::Cow;

auth_roles_endpoint!();
auth_role_endpoint!();

pub struct EndpointRoleRoleID {
    pub auth_provider_name: String,
    pub role_name: String,
}

impl EndpointRoleRoleID {
    pub fn new(auth_provider_name: String, role_name: String) -> EndpointRoleRoleID {
        EndpointRoleRoleID{auth_provider_name, role_name}
    }
}

impl Endpoint for EndpointRoleRoleID {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        Cow::from(format!("v1/auth/{}/role/{}/role-id", &self.auth_provider_name, &self.role_name))
    }
}

pub struct EndpointRoleSecretID {
    pub auth_provider_name: String,
    pub role_name: String,
}

impl EndpointRoleSecretID {
    pub fn new(auth_provider_name: String, role_name: String) -> EndpointRoleSecretID {
        EndpointRoleSecretID{auth_provider_name, role_name}
    }
}

impl Endpoint for EndpointRoleSecretID {
    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        Cow::from(format!("v1/auth/{}/role/{}/role-id", &self.auth_provider_name, &self.role_name))
    }
}

