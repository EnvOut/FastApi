extern crate liquid;
extern crate mongodb;

use std::collections::HashMap;

use linked_hash_map::LinkedHashMap;
use serde_json::Value;

use crate::domain::data_provider::postgres_data_provider::PostgresDataProvider;
use crate::domain::data_provider::proxy_data_provider::ProxyDataProvider;
use crate::domain::data_provider::mongo_data_provider::provider::MongoDataProvider;

pub mod mongo_data_provider;
pub mod postgres_data_provider;
pub mod proxy_data_provider;
pub mod test_data_provider;
pub mod query_provider;
pub mod postgres_utils;

pub enum DataProviderResult {
    Single(HashMap<String, Value>),
    Multiple(Vec<HashMap<String, Value>>),
}

pub enum CalProperties {
    MongoDB {
        query: String,
        query_type: String,
        is_single: bool,
    },
    Postgres,
}

pub trait DataProvider {
    fn get_name(&self) -> String;
    // q: '{"some": some=1}', type: "find", is_single
    fn call(&self, properties: CalProperties, options: HashMap<String, Value>) -> Result<DataProviderResult, ()>;
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