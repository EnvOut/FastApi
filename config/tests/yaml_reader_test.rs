extern crate apigen_config;
extern crate apigen_contract;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rust_embed;

use std::collections::HashMap;

use apigen_contract::domain::RootConfig;

const FULL_YAML_DATA: &str = "
            endpoints:
              first:
                provider: httpbin
                path: /first

              first2:
                provider: httpbin
                path: /first2

              second:
                provider:
                  ref: postgres
                  sql: select * from users
                path: /customers
                method: GET

              third:
                provider:
                  ref: postgres
                  sql: select * from users
                  is_single: false
                path: /customers2

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
                   ";

#[test]
fn yaml_lib_test() {
    use yaml_rust::{YamlLoader, YamlEmitter};

    let docs = YamlLoader::load_from_str("[1, 2, 3]").unwrap();
    let doc = &docs[0]; // select the first document
    assert_eq!(doc[0].as_i64().unwrap(), 1); // access elements by index
}

mod tdd {
    use std::env::current_dir;
    use std::fs::File;
    use std::path::{Path, PathBuf};

    use apigen_config::yaml_reader::YamlReader;
    use apigen_contract::config_contract::ConfigReader;

    use crate::FULL_YAML_DATA;

    #[test]
    fn should_read_auth_providers() {
        let yaml_reader = YamlReader::from_string(FULL_YAML_DATA.into());
        let root_config = yaml_reader.read();
        assert!(root_config.auth_provider_registry.is_empty(), false)
    }

    #[test]
    fn should_read_data_providers() {
        let yaml_reader = YamlReader::from_string(FULL_YAML_DATA.into());
        let root_config = yaml_reader.read();
        assert!(root_config.data_provider_registry.is_empty(), false)
    }

    #[test]
    fn should_read_endpoints() {
        let yaml_reader = YamlReader::from_string(FULL_YAML_DATA.into());
        let root_config = yaml_reader.read();
        assert!(&root_config.endpoint_registry.is_empty(), false)
    }
}

mod yaml_lib {
    use yaml_rust::{YamlEmitter, YamlLoader};

    use crate::FULL_YAML_DATA;
    use std::ptr::hash;

    #[test]
    fn read_array() {
        use yaml_rust::{YamlLoader, YamlEmitter};

        let docs = YamlLoader::load_from_str("[1, 2, 3]").unwrap();
        let doc = &docs[0]; // select the first document
        assert_eq!(doc[0].as_i64().unwrap(), 1); // access elements by index
    }

    #[test]
    fn read_map() {
        let docs = YamlLoader::load_from_str(&FULL_YAML_DATA).unwrap();
        let doc = &docs[0];
        let endpoints = &doc["endpoints"];

        let first = &endpoints["first"];

        let path = &first["path"].as_str().unwrap();

        let path_2 = *&doc["endpoints"]["first"]["path"].as_str().unwrap();

        assert_eq!(path_2, "/first"); // access elements by index
    }

    #[test]
    fn read_map_example() {
        let s =
            "
            foo:
                - list1
                - list2
            bar:
                - 1
                - 2.0
            ";
        let docs = YamlLoader::load_from_str(s).unwrap();

        // Multi document support, doc is a yaml::Yaml
        let doc = &docs[0];

        // Debug support
        println!("{:?}", doc);

        // Index access for map & array
        assert_eq!(doc["foo"][0].as_str().unwrap(), "list1");
        assert_eq!(doc["bar"][1].as_f64().unwrap(), 2.0);

        // Chained key/array access is checked and won't panic,
        // return BadValue if they are not exist.
        assert!(doc["INVALID_KEY"][100].is_badvalue());
    }

    #[test]
    fn read_equals() {
        let data = "
        numeric:
         - X-ORG-HEADER=12345
        stringified:
         - X-ORG-HEADER: some
        parts:
         - Authorization: Basic YWxhZGRpbjpvcGVuc2VzYW1l
        ";
        let docs = YamlLoader::load_from_str(data).unwrap();
        let doc = &docs[0];

        println!("{:?}",doc);
        let numeric = doc["numeric"][0].as_str().unwrap();
        let stringified = doc["stringified"][0].as_str().unwrap();
        let parts = doc["parts"][0].as_hash().unwrap();

        println!("parts: {:?}",parts)
//        Hash(
//            {
//                String("numeric"): Array([String("X-ORG-HEADER=12345")]),
//                String("stringified"): Array([String("X-ORG-HEADER=some")]),
//                String("parts"): Array([Hash({String("Authorization"): String("Basic YWxhZGRpbjpvcGVuc2VzYW1l")})])
//            }
//        )


    }
}