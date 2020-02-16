use crate::domain::auth_provider::{AuthError, AuthProvider, AuthResponse, AuthStatus, Role};

pub struct ProxyAuthProvider {
    external_url: String
}

impl AuthProvider for ProxyAuthProvider {
    fn authenticate(&mut self) -> Result<(), AuthResponse> {
        Ok(())
    }

    fn authorize(&mut self, required_roles: Vec<Role>) -> Result<AuthStatus, AuthError> {

    }
}
