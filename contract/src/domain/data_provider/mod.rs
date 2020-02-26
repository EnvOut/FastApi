extern crate liquid;
extern crate mongodb;

use std::collections::HashMap;
use std::rc::Rc;

use linked_hash_map::LinkedHashMap;
use serde_json::Value;

use crate::domain::data_provider::mongo_data_provider::provider::MongoDataProvider;
use crate::domain::data_provider::postgres_data_provider::provider::PostgresDataProvider;
use crate::domain::data_provider::proxy_data_provider::ProxyDataProvider;
use crate::domain::data_provider::test_data_provider::TestDataProvider;

pub mod mongo_data_provider;
pub mod postgres_data_provider;
pub mod proxy_data_provider;
pub mod test_data_provider;
pub mod query_provider;

pub enum DataProviderResult {
    Single(Value),
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
    // fn call(&self, properties: CalProperties, options: Value) -> Result<DataProviderResult, ()>;
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
    proxy_list: Vec<&'static ProxyDataProvider>,
    postgres_list: Vec<&'static PostgresDataProvider>,
    mongo_list: Vec<&'static MongoDataProvider>,
    test_data_list: Vec<&'static TestDataProvider>,
}

impl DataProviderRegistry {
    pub fn new() -> DataProviderRegistry {
        DataProviderRegistry {
            key_value_idx: HashMap::new(),
            proxy_list: vec![],
            postgres_list: vec![],
            mongo_list: vec![],
            test_data_list: vec![],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }

    pub fn add_proxy(&mut self, name: &str, value: ProxyDataProvider) {
        self.proxy_list.push(&value);

        let boxed = Box::new(value);
        self.key_value_idx.insert(name.to_string(), boxed);
    }
    pub fn add_postgres(&mut self, name: &str, value: PostgresDataProvider) {
        self.postgres_list.push(&value);

        let boxed = Box::new(value);
        self.key_value_idx.insert(name.to_string(), boxed);
    }
    pub fn add_mongo(&mut self, name: &str, value: MongoDataProvider) {
        self.mongo_list.push(&value);

        let boxed = Box::new(value);
        self.key_value_idx.insert(name.to_string(), boxed);
    }
    pub fn add_test(&mut self, name: &str, value: TestDataProvider) {
        self.test_data_list.push(&value);

        let boxed = Box::new(value);
        self.key_value_idx.insert(name.to_string(), boxed);
    }

    pub fn add(&mut self, name: &str, association: ProviderAssociation) -> &mut Self {
        match association {
            ProviderAssociation::Postgres(provider) => self.add_postgres(name, provider),
            ProviderAssociation::Mongo(provider) => self.add_mongo(name, provider),
            ProviderAssociation::Proxy(provider) => self.add_proxy(name, provider),
            ProviderAssociation::Test(provider) => self.add_test(name, provider),
            _ => unimplemented!()
        };
        self
    }
}

pub struct DataProviderCallProperties {}

pub type DateProperties = DataProviderCallProperties;

pub enum ProviderAssociation {
    Mongo(MongoDataProvider),
    Postgres(PostgresDataProvider),
    Proxy(ProxyDataProvider),
    Test(TestDataProvider),
}

impl ProviderAssociation {
    pub fn new_test() -> ProviderAssociation {
        ProviderAssociation::Test(TestDataProvider {})
    }
}