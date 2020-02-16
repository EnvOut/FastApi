use std::time::Duration;
use std::time::SystemTime;

use core_extensions::integer_extensions::ToTime;
use core_extensions::std_::collections::HashMap;
//extern crate serde;
use isahc::prelude::*;
use serde::{de, Deserialize, Deserializer};

use crate::domain::auth_provider::{AuthError, AuthProvider, AuthResponse, AuthStatus, Role, Roles};

use super::isahc::{http, RequestExt, ResponseExt};
use super::isahc::prelude::Request;

pub struct ProxyTokenAuthProvider {
    url_format: String,
    static_token: String,

    last_authenticated_at: SystemTime,
    update_authenticate_duration: Duration,
    roles: Roles,
}

impl AuthProvider for ProxyTokenAuthProvider {
    fn authenticate(&mut self) -> Result<(), AuthResponse> {
        fn prepare_external_link(format: &String, token: &String) -> String {
            let mut vars = HashMap::new();
            vars.insert("token".into(), token);
            strfmt::strfmt(format, &vars).unwrap()
        }

        let link = prepare_external_link(&self.url_format, &self.static_token);

        let mut response = Request::post(link)
            .header("Authorization", format!("Bearer {}", &self.static_token))
            // response should be in json format
            .header("Accept", "application/json")
            .body(())
            .unwrap()
            .send()
            .unwrap();

//        match response.status().as_u16() {
//            200 => (),
//            _ => ()
//        }
        let mut roles = match response.status().as_u16() {
            200 => response.json::<TokenProxyResponse>().map_err(|e| ()),
            _ => Err(())
        }.unwrap()
            .roles;
        self.roles = roles;
//        let result = AuthResponse { status: AuthStatus::Success, roles: roles.to_vec() };
        Ok(())
    }

    fn authorize(&mut self, required_roles: Vec<Role>) -> Result<AuthStatus, AuthError> {
        match (self.last_authenticated_at + self.update_authenticate_duration).elapsed() {
            Ok(_) => {
                self.authenticate();
            }
            _ => {}
        }

        fn check_roles(roles: &Roles, required_roles: &Roles) -> bool {
            required_roles.into_iter()
                .all(|it| roles.contains(it))
        }

        match check_roles(&self.roles, &required_roles) {
            true => Ok(AuthStatus::Success),
            false => Ok(AuthStatus::WrongRole { supported: self.roles.to_vec(), provided: required_roles.to_vec() })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt;
    use std::time::{Duration, SystemTime};

    use core_extensions::integer_extensions::ToTime;
    use core_extensions::std_::collections::hash_map::RandomState;
    use core_extensions::std_::env::var;

    #[test]
    fn elapsed_should_return_error() {
        let time = SystemTime::now();
        let plus5: Duration = 1.minutes();

        let elapsed = (time + plus5).elapsed().err().unwrap();

        println!("elapsed duration {:?}", elapsed)
    }

    #[test]
    fn wrong_format_should_be_silent() {
        let mut vars: HashMap<String, &str, RandomState> = HashMap::new();
        vars.insert("token".into(), "");

        let fmt = "some/".to_string();
        assert_eq!(strfmt::strfmt(&fmt, &vars).unwrap(), "some/")
    }

    #[test]
    fn format_works() {
        let mut vars = HashMap::new();
        vars.insert("token".to_string(), "12345");

        let fmt = "some/{token}".to_string();
        assert_eq!(strfmt::strfmt(&fmt, &vars).unwrap(), "some/12345")
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct TokenProxyResponse {
    roles: Roles,
}
