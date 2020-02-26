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
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
        let value = serde_json::to_value(data).unwrap();
        let result = DataProviderResult::Single(value);
        Ok(result)
    }
}
