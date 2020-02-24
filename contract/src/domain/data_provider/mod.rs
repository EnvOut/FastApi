extern crate liquid;

use std::collections::HashMap;
use std::iter::Map;

use linked_hash_map::LinkedHashMap;

use crate::domain::data_provider::mongo_data_provider::MongoDataProvider;
use crate::domain::data_provider::postgres_data_provider::PostgresDataProvider;
use crate::domain::data_provider::proxy_data_provider::ProxyDataProvider;

pub mod mongo_data_provider;
pub mod postgres_data_provider;
pub mod proxy_data_provider;
pub mod test_data_provider;
pub mod query_provider;
pub mod postgres_utils;

pub trait DataProvider {
    fn get_name(&self) -> String;
    fn call(&self, properties: Map<String, String>) -> Result<(), ()>;
}

pub struct DataContext {
    // name to results
    queries_results: LinkedHashMap<String, LinkedHashMap<String, String>>,
    // name to variable
    variables: LinkedHashMap<String, liquid::value::Value>,
}


#[warn(dead_code)]
pub struct DataProviderRegistry {
    key_value_idx: HashMap<String, Box<dyn DataProvider>>,
    proxy_list: Vec<ProxyDataProvider>,
    postgres_list: Vec<PostgresDataProvider>,
    mongo_list: Vec<MongoDataProvider>,
}

impl DataProviderRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }
}

pub struct DataProviderCallProperties {}

pub type DateProperties = DataProviderCallProperties;