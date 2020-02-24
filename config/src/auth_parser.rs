use yaml_rust::Yaml;

use apigen_contract::domain::auth_provider::{AuthProvider, AuthProviderRegistry};

pub fn read_auth(auth_doc: &Yaml) -> Box<dyn AuthProvider> {
    let kind = &auth_doc["kind"];
    // let rr = match kind.as_str().unwrap() {
    //     "basic" => "",
    //     _ => {}
    // };
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
        kind: basic
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

      self_oauth2base:
        kind: oauth2base
        credential:
            username:
                some
            password:
                qwerty
        provider: "some.provider.com/auth"

    "#;

    #[test]
    fn should_return_auths() {
        simple_logger::init().unwrap();

        let docs = YamlLoader::load_from_str(YAML_DATA).unwrap();
        let doc = &docs[0];

        println!("{:?}", doc);

        let transforms_doc = &doc["auth"];

        let registry = read_auths(transforms_doc);
    }

    #[test]
    fn should_parse_basic_auth() {
        let docs = YamlLoader::load_from_str(YAML_DATA).unwrap();
        let doc = &docs[0];

        println!("{:?}", doc);

        let siber_static_base = &doc["auth"]["siber_static_base"];
    }
}