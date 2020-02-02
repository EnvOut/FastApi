use crate::domain::auth_provider::AuthProviderRegistry;
use crate::domain::data_provider::DataProviderRegistry;
use crate::domain::endpoint::EndpointRegistry;
use crate::domain::transformer_provider::TransformerProviderRegistry;

pub mod data_provider;
pub mod auth_provider;
pub mod endpoint;
pub mod transformer_provider;

pub struct RootConfig {
    pub endpoint_registry: EndpointRegistry,
    pub data_provider_registry: DataProviderRegistry,
    pub auth_provider_registry: AuthProviderRegistry,
    pub transformer_provider_registry: TransformerProviderRegistry,
}