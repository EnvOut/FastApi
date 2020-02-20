use core_extensions::std_::time::{SystemTime, Duration};

use crate::domain::auth_provider::{AppToken, AuthError, AuthProvider, AuthResponse, AuthStatus, BaseCredential, Role};
use crate::domain::auth_provider::proxy_token_auth::ProxyTokenAuthProvider;
use core_extensions::integer_extensions::ToTime;

pub enum TokenTypes {
    Bearer,
    MAC,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SomeJsonStruct {
    name: String,
    path: String,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct OAuthToken {
    access_token: String,
    refresh_token: String,
    #[serde(default = "TokenTypes::Bearer")]
    token_type: TokenTypes,
    // default is now + half year
    #[serde(default = "SystemTime::now() + 4464.hours()")]
    expires_at: SystemTime,
    scopes: [String],
}

struct OAuthBaseTokenResponse {}

pub struct OAuth2BaseAuthProvider {
    credential: BaseCredential,
    token: Option<OAuthToken>,
    external_url: String,
}

impl OAuth2BaseAuthProvider {
    fn new(credential: BaseCredential, external_url: String) -> OAuth2BaseAuthProvider {
        OAuth2BaseAuthProvider {
            credential,
            token: None,
            external_url,
        }
    }
}

impl AuthProvider for OAuth2BaseAuthProvider {
    fn authenticate(&mut self) -> Result<(), AuthResponse> {
        let BaseCredential { login, password } = &self.credentials;

        let response = || isahc::get(&self.external_url).unwrap();
        let status = status().as_u16();

        let x1 = match status() {
            200 => {
                let x = Ok(response.json::<TokenProxyResponse>().map_err(|e| ()));
            }
            302 | 401 => Err(AuthError::WrongStatusCode),
            _ => Err(AuthError::NotImplementedStatusCode)
        };
    }

    fn authorize(&mut self, required_roles: Vec<Role>) -> Result<AuthStatus, AuthError> {
        unimplemented!()
    }
}
