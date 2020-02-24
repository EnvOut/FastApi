use std::iter::Map;

use core_extensions::std_::collections::HashMap;

use crate::domain::auth_provider::{AuthProvider, AuthStatus};
use crate::domain::data_provider::DataProvider;
use crate::domain::endpoint::AbstractEndpointProvider;
use crate::domain::web::{HttpMethods, Request, Response};

pub struct PermissionConfig {
    resource_permissions: Vec<String>,
    method_permissions: HashMap<HttpMethods, String>,
}

impl PermissionConfig {
    fn get_permissions(&self, method: &HttpMethods) -> Vec<String> {
        match self.method_permissions.get_key_value(method) {
            Some(value) => &self.resource_permissions + value,
            _ => self.resource_permissions.to_vec()
        }
    }
}

pub struct CrudEndpointProvider {
    data_provider: Box<dyn DataProvider>,
    data_provider_properties: Map<String, String>

    auth_provider: Box < dyn AuthProvider >,

    auth_config: PermissionConfig,
}

impl AbstractEndpointProvider for CrudEndpointProvider {
    fn handle(&mut self, req: &Request) -> &Response {
        let auth_permissions = self.auth_config.get_permissions(&req.method);
        match *self.auth_provider.authorize(auth_permissions) {
            AuthStatus::Success => *self.data_provider.call()
            _ => ()
        }

        // & self.auth_provider.authorize()
        // & self.data_provider.call()
    }
}



