use crate::domain::data_provider::{DataProvider, DataProviderResult, CalProperties};
use core_extensions::std_::collections::hash_map::RandomState;
use core_extensions::std_::collections::HashMap;
use serde_json::Value;

pub struct TestDataProvider {}

impl DataProvider for TestDataProvider {
    fn get_name(&self) -> String {
        "test_data_provider".into()
    }

    fn call(&self, properties: CalProperties, options: HashMap<String, Value, RandomState>) -> Result<DataProviderResult, ()> {
        unimplemented!()
    }
}