extern crate isahc;
extern crate strfmt;

use std::collections::HashMap;

use serde::{de, Deserialize, Deserializer};


//pub mod proxy_auth;
pub mod base_auth;
pub mod oauth2base_auth;
pub mod proxy_token_auth;

pub trait AuthProvider {
    fn authenticate(&mut self) -> Result<(), AuthResponse>;
    fn authorize(&mut self, required_roles: Roles) -> Result<AuthStatus, AuthError>;
}

#[warn(dead_code)]
pub struct AuthProviderRegistry {
    key_value_idx: HashMap<String, Box<dyn AuthProvider>>,
//    proxy_auth_idx: Vec<ProxyAuthProvider>,
//    static_auth_idx: Vec<StaticAuthProvider>,
}

impl AuthProviderRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    name: String
}

pub type Roles = Vec<Role>;

pub enum AuthStatus {
    Success,
    Denied,
    WrongRole { supported: Roles, provided: Roles },
}

pub struct AuthResponse {
    status: AuthStatus,
    roles: Roles,
}

pub enum AuthException {
    LoginException,
    AccessDenied,
    Forbidden,
}

pub enum AuthError {
    NotImplementedStatusCode,
    WrongStatusCode,
    CantParseResponse
}

pub struct BaseCredential {
    login: String,
    password: String,
}

pub struct AppToken {
    token: String
}