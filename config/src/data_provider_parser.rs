use std::rc::Rc;

use yaml_rust::Yaml;

use apigen_contract::domain::data_provider::{DataProvider, DataProviderRegistry, ProviderAssociation};
use apigen_contract::domain::data_provider::test_data_provider::TestDataProvider;

pub fn read_data_providers(doc: &Yaml) -> DataProviderRegistry {
    let mut registry = DataProviderRegistry::new();

    let providers = &doc["providers"];

    providers.as_hash().unwrap()
        .into_iter()
        .map(|(name, def)| (name.as_str().unwrap(), read_provider(def)))
        .for_each(|(name, association)| {
            registry.ad(name, association);
        });

    registry
}

pub fn read_provider(provider_doc: &Yaml) -> ProviderAssociation {
    let association = ProviderAssociation::new_test();
    association
}

mod tests {
    static YAML_STR: &str = r#"
        providers:
          httpbin:
            type: proxy
            path: httpbin.org/get
            shema: https
            allowed:
              methods:
                - GET
                - POST
              parameters:
                - cat
                - param1

          postgres:
            type: postgres
            user:
              qwerty
            password:
              qwerty
            db:
              new
            port:
              10432
            allowed:
              parameters: false
               "#;

    #[test]
    fn test_it() {}
}