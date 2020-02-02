use crate::domain::RootConfig;
use std::path::Path;

pub trait ConfigReader<T> {
    fn new(config_path: &Path) -> T;
    fn read(&self) -> &RootConfig;
}

pub struct DataProviderDefinition {}

pub struct EndpointDefinition {}

pub struct AuthProviderDefinition {}
