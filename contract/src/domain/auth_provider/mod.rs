use std::collections::HashMap;

pub trait AuthProvider {}

pub struct ProxyAuthProvider {}

impl AuthProvider for ProxyAuthProvider {}

pub struct StaticAuthProvider {}

impl AuthProvider for StaticAuthProvider {}

pub struct AuthProviderRegistry {
    key_value_idx: HashMap<String, Box<dyn AuthProvider>>,
    proxy_auth_idx: Vec<ProxyAuthProvider>,
    static_auth_idx: Vec<StaticAuthProvider>,
}

impl AuthProviderRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }
}