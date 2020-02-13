use std::collections::HashMap;
use std::iter::Map;

use crate::domain::data_provider::mongo_data_provider::MongoDataProvider;
use crate::domain::data_provider::postgres_data_provider::PostresDataProvider;
use crate::domain::data_provider::proxy_data_provider::ProxyDataProvider;

pub mod mongo_data_provider;
pub mod postgres_data_provider;
pub mod proxy_data_provider;

pub trait DataProvider {
    fn get_name(&self) -> String;
    fn call(&self, properties: Map<String, String>) -> Result<(), ()>;
}

#[warn(dead_code)]
pub struct DataProviderRegistry {
    key_value_idx: HashMap<String, Box<dyn DataProvider>>,
    proxy_list: Vec<ProxyDataProvider>,
    postgres_list: Vec<PostresDataProvider>,
    mongo_list: Vec<MongoDataProvider>,
}

impl DataProviderRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }
}