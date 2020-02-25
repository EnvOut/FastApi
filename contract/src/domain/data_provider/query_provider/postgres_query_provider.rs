use linked_hash_map::LinkedHashMap;

use crate::domain::data_provider::DataContext;
use crate::domain::data_provider::query_provider::QueryProvider;
use liquid::Template;

extern crate liquid;

pub struct PostgresQueryProvider {
    parameters: LinkedHashMap<String, String>,
    query_template: Template,
}

impl QueryProvider for PostgresQueryProvider {
    fn prepare_query(&mut self, _context: DataContext) -> String {
        let template = liquid::ParserBuilder::with_liquid()
            .build()
            .parse("Liquid! {{num | minus: 2}}").unwrap();

        let mut globals = liquid::value::Object::new();
        globals.insert("num".into(), liquid::value::Value::scalar(4f64));

        let output = template.render(&globals).unwrap();
        assert_eq!(output, "Liquid! 2".to_string());

        unimplemented!()
    }

    fn new() -> Self {
        unimplemented!()
    }
}