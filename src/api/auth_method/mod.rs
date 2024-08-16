pub mod aws;
pub mod approle;
pub mod jwt_oidc;

#[macro_export]
macro_rules! auth_config_endpoint {
    () => {
        pub struct EndpointConfig {
            pub auth_provider_name: String,
        }

        impl EndpointConfig {
            pub fn new(auth_provider_name: String) -> EndpointConfig {
                EndpointConfig{auth_provider_name}
            }
        }

        impl crate::api::endpoint::Endpoint for EndpointConfig {
            fn method(&self) -> http::Method {
                http::Method::GET
            }

            fn endpoint(&self) -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::from(format!("v1/auth/{}/config", self.auth_provider_name))
            }
        }
    };
}

#[macro_export]
macro_rules! auth_role_endpoint {
    () => {
        pub struct EndpointRole {
            pub auth_provider_name: String,
            pub role_name: String
        }

        impl EndpointRole {
            pub fn new(auth_provider_name: String, role_name: String) -> EndpointRole {
                EndpointRole{auth_provider_name, role_name}
            }
        }

        impl crate::api::endpoint::Endpoint for EndpointRole {
            fn method(&self) -> http::Method {
                http::Method::GET
            }

            fn endpoint(&self) -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::from(format!("v1/auth/{}/role/{}", self.auth_provider_name, self.role_name))
            }
        }
    };
}

#[macro_export]
macro_rules! auth_roles_endpoint {
    () => {
        pub struct EndpointRoles {
            pub auth_provider_name: String,
        }

        impl EndpointRoles {
            pub fn new(auth_provider_name: String) -> EndpointRoles {
                EndpointRoles{auth_provider_name}
            }
        }

        impl crate::api::endpoint::Endpoint for EndpointRoles {
            fn method(&self) -> http::Method {
                http::Method::GET
            }

            fn endpoint(&self) -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::from(format!("v1/auth/{}/role/?list=true", self.auth_provider_name))
            }
        }
    };
}
