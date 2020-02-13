use crate::domain::data_provider::DataProvider;
use std::iter::Map;

pub struct PostresDataProvider{}
impl DataProvider for PostresDataProvider{
    fn get_name(&self) -> String {
        "postgres".into()
    }

    fn call(&self, _properties: Map<String, String>) -> Result<(), ()> {
        unimplemented!()
    }
}