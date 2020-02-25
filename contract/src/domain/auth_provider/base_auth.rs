use crate::domain::auth_provider::{AuthError, AuthProvider, AuthResponse, AuthStatus, BaseCredential, Roles};

//#[derive(Eq, PartialEq)]
pub struct BaseAuthProvider {
    credentials: BaseCredential,
    roles: Roles,
    external_url: String,
}

impl AuthProvider for BaseAuthProvider {
    fn authenticate(&mut self) -> Result<(), AuthResponse> {
        Ok(())
    }

    fn authorize(&mut self, required_roles: Roles) -> Result<AuthStatus, AuthError> {
        let BaseCredential { login: _, password: _ } = &self.credentials;

        fn check_roles(roles: &Roles, required_roles: &Roles) -> bool {
            required_roles.into_iter()
                .all(|i| roles.contains(i))
        }

        let status = || isahc::get(&self.external_url).unwrap().status().as_u16();

        match check_roles(&self.roles, &required_roles) {
            true => match status() {
                200 => Ok(AuthStatus::Success),
                302 | 401 => Ok(AuthStatus::Denied),
                _ => Err(AuthError::NotImplementedStatusCode)
            },
            false => Ok(AuthStatus::WrongRole { supported: self.roles.to_vec(), provided: required_roles.to_vec() })
        }
    }
}

