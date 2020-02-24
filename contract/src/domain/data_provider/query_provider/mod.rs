extern crate liquid;

use crate::domain::data_provider::DataContext;

pub mod postgres_query_provider;
pub mod mongo_query_provider;

pub trait QueryProvider {
    fn prepare_query(&mut self, context: DataContext) -> String;
    fn new() -> Self;
}