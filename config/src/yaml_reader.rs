extern crate apigen_contract;
extern crate yaml_rust;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use apigen_contract::config_contract::ConfigReader;

use self::yaml_rust::{YamlLoader, Yaml};
use self::apigen_contract::domain::RootConfig;
use self::apigen_contract::domain::endpoint::EndpointRegistry;

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
        println!("data: {:?}", self.data);
        unimplemented!()
    }
}

fn read_endpoints(doc: &Yaml)->EndpointRegistry {
    let endpoints = &doc["endpoints"];

    let first = &endpoints["first"];

    let path = &first["path"].as_str().unwrap();

    let path_2 = *&doc["endpoints"]["first"]["path"].as_str().unwrap();
    unimplemented!()
}


#[cfg(test)]
mod tests {
    #[test]
    fn some() -> () {
        ()
    }
}