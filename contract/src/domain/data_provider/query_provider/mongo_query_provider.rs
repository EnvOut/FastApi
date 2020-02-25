use crate::domain::data_provider::DataContext;
use crate::domain::data_provider::query_provider::QueryProvider;

pub struct MongoQueryProvider{

}

impl QueryProvider for MongoQueryProvider {
    fn prepare_query(&mut self, _context: DataContext) -> String {
        unimplemented!()
    }

    fn new() -> Self {
        unimplemented!()
    }
}