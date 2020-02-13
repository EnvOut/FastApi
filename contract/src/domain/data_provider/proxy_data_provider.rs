use crate::domain::data_provider::DataProvider;
use std::iter::Map;

pub struct ProxyDataProvider{}
impl DataProvider for ProxyDataProvider{
    fn get_name(&self) -> String {
        "proxy".into()
    }

    fn call(&self, _properties: Map<String, String>) -> Result<(), ()> {
        unimplemented!()
    }
}