use postgres::{Client, Column, Config, Row};

use crate::domain::data_provider::{CalProperties, DataProvider, DataProviderResult};
use core_extensions::std_::collections::hash_map::RandomState;
use core_extensions::std_::collections::HashMap;
use serde_json::Value;

pub struct PostgresDataProvider {
    config: Config,
    client: Client,
}

impl DataProvider for PostgresDataProvider {
    fn get_name(&self) -> String {
        "postgres".into()
    }

    fn call(&self, properties: CalProperties, _options: HashMap<String, Value, RandomState>) -> Result<DataProviderResult, ()> {
        match properties {
            CalProperties::Postgres => unimplemented!(),
            _ => Err(())
        }
    }
}

mod tests {
    use crate::domain::data_provider::postgres_utils::row_to_map;
    use std::collections::HashMap;
    use serde_json::Value;
    use postgres::Config;

    #[test]
    fn try_pg_driver() {
        use postgres::{Client, NoTls};

        let mut config: Config = Client::configure();
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