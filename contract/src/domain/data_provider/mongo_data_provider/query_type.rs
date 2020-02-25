use crate::domain::data_provider::mongo_data_provider::query_type::MongoQueryType::*;

pub enum MongoQueryType {
    Find,
    Insert,
    Aggregate,
    MapReduce,
    Distinct,
    Delete,
    BulkWrite,
    FindAnd,
}

impl MongoQueryType {
    pub fn from_str(value: String) -> MongoQueryType {
        match value.to_lowercase().as_str() {
            "find" => Find,
            "insert" => Insert,
            "aggregate" => Aggregate,
            "mapreduce" => MapReduce,
            "distinct" => Distinct,
            "delete" => Delete,
            "bulkwrite" => BulkWrite,
            "findand" => FindAnd,
            _ => unimplemented!()
        }
    }
}