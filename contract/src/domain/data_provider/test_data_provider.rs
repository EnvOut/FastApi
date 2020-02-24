use crate::domain::data_provider::DataProvider;
use core_extensions::std_::iter::Map;

pub struct TestDataProvider {

}

impl DataProvider for TestDataProvider {
    fn get_name(&self) -> String {
        "test_data_provider".into()
    }

    fn call(&self, properties: Map<String, String>) -> Result<(), ()> {
unimplemented!()
    }
}