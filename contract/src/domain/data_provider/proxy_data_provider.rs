use core_extensions::std_::collections::hash_map::RandomState;
use core_extensions::std_::collections::HashMap;
use serde_json::Value;

use crate::domain::data_provider::{CalProperties, DataProvider, DataProviderResult};

pub struct ProxyDataProvider{}
impl DataProvider for ProxyDataProvider{
    fn get_name(&self) -> String {
        "proxy".into()
    }

    fn call(&self, properties: CalProperties, options: HashMap<String, Value, RandomState>) -> Result<DataProviderResult, ()> {
        unimplemented!()
    }
}