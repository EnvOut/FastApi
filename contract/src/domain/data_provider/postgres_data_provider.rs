use std::error::Error;
use std::iter::Map;
use std::sync::Arc;

use core_extensions::std_::collections::HashMap;
use core_extensions::TypeIdentity;
use postgres::{Column, Row};
use postgres::types::Type;
use serde::de::Unexpected::Other;
use serde_json::json;
use serde_json::value::Value;
use serde_postgres::Deserializer;

use crate::domain::data_provider::DataProvider;

pub struct PostgresDataProvider {}

impl DataProvider for PostgresDataProvider {
    fn get_name(&self) -> String {
        "postgres".into()
    }

    fn call(&self, _properties: Map<String, String>) -> Result<(), ()> {
        unimplemented!()
    }
}

mod tests {
    use std::any::Any;

    use core_extensions::std_::collections::HashMap;
    use postgres::Column;
    use serde_json::Value;

    use crate::domain::data_provider::postgres_utils::row_to_map;

    #[test]
    fn try_pg_driver() {
        use postgres::{Client, NoTls};

        let mut config = Client::configure();
        config.host("localhost");
        config.port(54320);
        config.dbname("postgres");
        config.user("postgres");
        config.password("postgres");

        let mut client = config.connect(NoTls).unwrap();

        let name = "Ferris";
        let data = None::<&[u8]>;

        client.execute(
            "INSERT INTO person (name, data) VALUES ($1, $2)",
            &[&name, &data],
        );

        {
            let query = "SELECT id, name, data FROM person";

            let items: Vec<HashMap<String, Value>> = client.query(query, &[]).unwrap().into_iter()
                .map(|row| row_to_map(row))
                .collect();

            println!("{:?}", &items)
        }
    }
}