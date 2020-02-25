use std::collections::HashMap;

use serde_json::Value;

use super::super::mongodb::options::*;

pub struct MongoOption {
    source_data: HashMap<String, Value>
}

impl MongoOption {
    pub fn new(data: HashMap<String, Value>) -> MongoOption {
        MongoOption { source_data: data }
    }

    pub fn find(&mut self) -> Option<FindOptions> {
        None
    }
    pub fn find_one(&mut self) -> Option<FindOneOptions> {
        None
    }
    pub fn find_one_and_delete(&mut self) -> Option<FindOneAndDeleteOptions> {
        None
    }
    pub fn find_one_and_update(&mut self) -> Option<FindOneAndUpdateOptions> {
        None
    }
    pub fn find_one_and_replace(&mut self) -> Option<FindOneAndReplaceOptions> {
        None
    }
    pub fn insert_one(&mut self) -> Option<InsertOneOptions> {
        None
    }
    pub fn insert_many(&mut self) -> Option<InsertManyOptions> {
        None
    }
    pub fn replace_one(&mut self) -> Option<ReplaceOptions> {
        None
    }
    pub fn update_one(&mut self) -> Option<UpdateOptions> {
        None
    }
    pub fn update_many(&mut self) -> Option<UpdateOptions> {
        None
    }
    pub fn aggregate(&mut self) -> Option<AggregateOptions> {
        None
    }
    pub fn distinct(&mut self) -> Option<DistinctOptions> {
        None
    }
    pub fn delete_one(&mut self) -> Option<DeleteOptions> {
        None
    }
    pub fn delete_many(&mut self) -> Option<DeleteOptions> {
        None
    }
    pub fn count_documents(&mut self) -> Option<CountOptions> {
        None
    }
}