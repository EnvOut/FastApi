use core_extensions::SelfOps;
use core_extensions::std_::collections::hash_map::RandomState;
use core_extensions::std_::collections::HashMap;
use isahc::version;
use serde_json::Value;

use crate::domain::data_provider::{CalProperties, DataProvider, DataProviderResult};

pub struct TestDataProvider {}

impl DataProvider for TestDataProvider {
    fn get_name(&self) -> String {
        "test_data_provider".into()
    }

    fn call(&self, properties: CalProperties, options: HashMap<String, Value>) -> Result<DataProviderResult, ()> {
    // fn call(&self, properties: CalProperties, options: Value) -> Result<DataProviderResult, ()>{
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let mut value = serde_json::to_value(data).unwrap();
        // options.
        options.into_iter()
            .for_each(|(k, v)| {
                value[k] = v;
            });

        let result = DataProviderResult::Single(value);
        Ok(result)
    }
}
