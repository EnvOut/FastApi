use yaml_rust::Yaml;

use apigen_contract::domain::auth_provider::AuthProviderRegistry;

pub fn read_auth(auth_doc: &Yaml) -> AuthProviderRegistry {
    unimplemented!()
}

pub fn read_auths(auth_docs: &Yaml) -> AuthProviderRegistry {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use yaml_rust::YamlLoader;

    use crate::auth_parser::read_auths;
    use crate::yaml_reader::YamlReader;

    const YAML_DATA: &str = r#"
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
    "#;

    #[test]
    fn should_return_auths() {
        simple_logger::init().unwrap();

        let docs = YamlLoader::load_from_str(YAML_STR).unwrap();
        let doc = &docs[0];

        println!("{:?}", doc);

        let transforms_doc = &doc["auth"];

        let registry = read_auths(transforms_doc);
    }
}