extern crate apigen_contract;
extern crate simple_logger;
extern crate yaml_rust;

use std::collections::hash_map::RandomState;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::iter::Map;
use std::path::Path;
use std::slice::Iter;

use json::JsonValue;
use json::number::Number;

use apigen_contract::config_contract::ConfigReader;

use self::apigen_contract::domain::endpoint::EndpointRegistry;
use self::apigen_contract::domain::RootConfig;
use self::apigen_contract::domain::transformer_provider::chain_transformer::RequestTransformChain;
use self::apigen_contract::domain::transformer_provider::chain_transformer::RequestTransformers;
use self::apigen_contract::domain::transformer_provider::TransformerProviderRegistry;
use self::yaml_rust::{Yaml, YamlLoader};
use self::yaml_rust::yaml::Hash;

pub struct YamlReader {
    data: String
}

impl YamlReader {
    pub fn from_string(data: String) -> YamlReader {
        YamlReader { data }
    }
}

impl ConfigReader<YamlReader> for YamlReader {
    fn new(config_path: &Path) -> YamlReader {
        let mut data: String = "".into();
        let file = fs::File::open(&config_path)
//        let file = fs::File::open(&config_path)
            .map(|mut f| f.read_to_string(&mut data))
            .ok().expect(format!("Can't find config file with path: \"{:?}\"", &config_path).as_str());
        let abs = fs::canonicalize(&config_path);
        println!("abs: {:?}", abs.unwrap());
        YamlReader { data }
    }

    fn read(&self) -> &RootConfig {
        unimplemented!()
    }
}

fn read_endpoints(doc: &Yaml) -> EndpointRegistry {
    let endpoints = &doc["endpoints"];

    let first = &endpoints["first"];

    let path = &first["path"].as_str().unwrap();

    let path_2 = *&doc["endpoints"]["first"]["path"].as_str().unwrap();
    unimplemented!()
}

fn read_transformers(doc: &Yaml) -> TransformerProviderRegistry {
    let mut registry = TransformerProviderRegistry::new();

    &doc["transforms"].as_hash().unwrap()
        .into_iter()
        .map(|(name, def)| (name.as_str().unwrap(), read_transformer(def)))
        .for_each(|(name, tr)| { registry.add(&name.to_owned(), tr); });

    registry
}

fn read_transformer(transformer_doc: &Yaml) -> RequestTransformChain {
    println!("{:?}", transformer_doc);

    let mut chain = RequestTransformChain::new();


    fn extract_key_value_lambda(hash: &Hash) -> (&String, JsonValue) {
        log::debug!("extract_key_value_lambda.hash: {:?}", hash);
        let item = hash
            .into_iter()
            .map(|(k, v)| (extract_value_lambda(k), to_json_value(v)))
            .find(|_| true)
            .unwrap();
        log::debug!("extract_key_value_lambda.item: {:?}", item);
        item
    }

    fn extract_value_lambda(doc: &Yaml) -> &String {
        log::trace!("extract_value_lambda: {:?}", doc);
        match doc {
            Yaml::String(v) => v,
            _ => unimplemented!()
        }
    }

    fn to_json_value(doc: &Yaml) -> JsonValue {
        match doc {
            Yaml::Real(real_string) => {
                let parsed: f64 = real_string.parse().unwrap();
                let number = Number::from(parsed);
                JsonValue::Number(number)
            }
            Yaml::Integer(value) => JsonValue::Number(Number::from(value.clone() as f64)),
            Yaml::String(v) => JsonValue::String(v.clone()),
            Yaml::Boolean(v) => JsonValue::Boolean(v.clone()),
            Yaml::Null => JsonValue::Null,
            _ => unimplemented!()
        }
    }

    transformer_doc.as_hash().unwrap()
        .into_iter()
        .flat_map(|(target_def, operations)| {
            let target_name = target_def.as_str().unwrap();
            operations.as_hash().unwrap().into_iter()
                .map(|(op_name, op_def)| (op_name.as_str().unwrap(), op_def.as_vec().unwrap()))
                .flat_map(move |(operation_name, items)| {
                    let operation: fn(&Yaml) -> RequestTransformers = match (target_name, operation_name) {
                        ("header", "add") => {
                            |doc| {
                                let hash = match doc {
                                    Yaml::Hash(hash) => hash,
                                    _ => unimplemented!()
                                };

                                let (k, v) = extract_key_value_lambda(&hash);
                                RequestTransformers::HeaderAdd(k.clone(), v.as_str().unwrap().to_string())
                            }
                        }
                        ("header", "remove") => {
                            |doc| {
                                let v = match doc {
                                    Yaml::String(v) => v,
                                    _ => unimplemented!()
                                }.clone();

                                RequestTransformers::HeaderRemove(v)
                            }
                        }
                        ("json", "add") => {
                            |doc| {
                                let hash = match doc {
                                    Yaml::Hash(hash) => hash,
                                    _ => unimplemented!()
                                };
                                let (k, v) = extract_key_value_lambda(hash);
                                RequestTransformers::JsonAdd(k.clone(), v)
                            }
                        }
                        ("json", "remove") => {
                            |doc| {
                                let v = match doc {
                                    Yaml::String(v) => v,
                                    _ => unimplemented!()
                                }.clone();
                                RequestTransformers::JsonRemove(v)
                            }
                        }
                        _ => unimplemented!()
                    };

                    let stream: Vec<RequestTransformers> = items.into_iter()
                        .map(operation)
                        .collect();
                    stream.into_iter()
                })
        })
        .fold(&mut chain, |c, t| c.with_transformer(t));

//    let chain = chain;
    chain
}

#[cfg(test)]
mod tests {
    use log::{info, trace, warn};

    use crate::yaml_reader::{read_transformer, read_transformers};

    use super::apigen_contract::domain::transformer_provider::chain_transformer::RequestTransformers;
    use super::yaml_rust::{Yaml, YamlLoader};

    static YAML_STR: &str = r#"
    endpoints:
      first:
        provider: httpbin
        path: /first
        auth:
          ref: mego_proxy
          roles:
            - user
            - admin

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

    auth:
      siber_static_base:
        kind: static
        provider: "some.provider.com/auth"
        base_auth:
          login: user
          password: qwerty

      siber_static_token:
        kind: static
        provider: "some.provider.com/auth"
        token: "some_super_token"

      mego_proxy:
        kind: proxy
        provider: "proxy.com/auth"

    transforms:
      postgres:
        header:
          add:
            - X-ORG-REG: token
          remove:
            - X-GARBAGE
        json:
          add:
            - name: Alexander Pushkin
            - age: 3
          remove:
            - name
    "#;

    #[test]
    fn some() -> () {}

    #[test]
    fn postgres_transformers_should_be_correct() {
        simple_logger::init().unwrap();

        let docs = YamlLoader::load_from_str(YAML_STR).unwrap();
        let doc = &docs[0];

        println!("{:?}", doc);

        let transforms_doc = &doc["transforms"];
        println!("transforms_doc {:?}", transforms_doc);
        let postgres_transformer = &transforms_doc["postgres"];

        let transformers = &read_transformer(postgres_transformer).get_transformers();
        let mut iter = transformers.iter();

        log::debug!("transformers: {:?}", transformers);
        assert_eq!(*iter.next().unwrap().clone(), RequestTransformers::HeaderAdd("X-ORG-REG".into(), "token".into()));
        assert_eq!(*iter.next().unwrap().clone(), RequestTransformers::HeaderRemove("X-GARBAGE".into()));
        assert_eq!(*iter.next().unwrap().clone(), RequestTransformers::JsonAdd("name".into(), "Alexander Pushkin".into()));
        assert_eq!(*iter.next().unwrap().clone(), RequestTransformers::JsonAdd("age".into(), 3.into()));
        assert_eq!(*iter.next().unwrap().clone(), RequestTransformers::JsonRemove("name".into()));
    }
}