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

use crate::transforms_parser::read_transformers;

use self::apigen_contract::domain::endpoint::EndpointRegistry;
use self::apigen_contract::domain::RootConfig;
use self::apigen_contract::domain::transformer_provider::chain_transformer::RequestTransformChain;
use self::apigen_contract::domain::transformer_provider::chain_transformer::RequestTransformers;
use self::apigen_contract::domain::transformer_provider::TransformerProviderRegistry;
use self::yaml_rust::{Yaml, YamlLoader};
use self::yaml_rust::yaml::Hash;
use crate::auth_parser::read_auths;

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
        let docs = YamlLoader::load_from_str(self.data.as_str()).unwrap();
        let doc = &docs[0];

        let transformer_provider_registry = read_transformers(doc);
        let auth_provider_registry = read_auths(doc);
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
