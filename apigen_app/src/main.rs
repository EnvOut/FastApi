extern crate apigen_config;
extern crate apigen_contract;

use std::path::{Path};
use apigen_config::yaml_reader::YamlReader;
use apigen_contract::config_contract::ConfigReader;

#[cfg(debug_assertions)]
const DEFAULT_CONFIG_PATH_STR: &str = "../resources/config/api-gen.yaml";
#[cfg(not(debug_assertions))]
const DEFAULT_CONFIG_PATH_STR: &str = "api-gen.yaml";

fn main() {
    let config_path = Path::new(DEFAULT_CONFIG_PATH_STR);

    println!("Hello, world!");
    let reader = YamlReader::new(config_path);
    reader.read();
}
