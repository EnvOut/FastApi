use std::collections::HashMap;

use serde_json::Value;

use crate::domain::data_provider::{CalProperties, DataProvider, DataProviderResult};
use crate::domain::data_provider::mongo_data_provider::query_option::MongoOption;
use crate::domain::data_provider::mongo_data_provider::query_type::MongoQueryType;

use super::super::mongodb::Client;
use super::super::mongodb::options::ClientOptions;

pub struct MongoDataProvider {
    client_options: ClientOptions,
    client: Client,
    database: String,
    collection: String,
}

impl DataProvider for MongoDataProvider {
    fn get_name(&self) -> String {
        "postgres".into()
    }

    fn call(&self, properties: CalProperties, options: HashMap<String, Value>) -> Result<DataProviderResult, ()> {
        let mut mongo_options = MongoOption::new(options);

        let collection = self.client.database(self.database.as_str()).collection(self.collection.as_str());


        // collection.find();
        // collection.find_one();
        //
        // collection.find_one_and_delete();
        // collection.find_one_and_update();
        // collection.find_one_and_replace();
        //
        // collection.insert_one();
        // collection.insert_many();
        //
        // collection.replace_one();
        //
        // collection.update_one();
        // collection.update_many();
        //
        // collection.aggregate();
        //
        // collection.distinct();
        //
        // collection.delete_one();
        // collection.delete_many();
        //
        // collection.count_documents();

        // collection.selection_criteria()
        // collection.read_concern()

        let data_provider_result: DataProviderResult = match properties {
            CalProperties::MongoDB { query, query_type, is_single } => {
                let json: Value = serde_json::from_str(query.as_str()).unwrap();
                let bson = bson::to_bson(&json).unwrap();

                let document = match bson {
                    bson::Bson::Document(document) => Some(document),
                    _ => None
                };

                match (MongoQueryType::from_str(query_type), is_single) {
                    (Find, singe) => {
                        let result: DataProviderResult = match singe {
                            true => {
                                let execution_result = collection.find_one(document, mongo_options.find_one()).expect("Document not found").unwrap();
                                let result: HashMap<String, Value> = bson::from_bson(bson::Bson::Document(execution_result)).unwrap();
                                DataProviderResult::Single(result)
                            }
                            false => {
                                // let execution_result = collection.find(document, mongo_options.find()).expect("Document not found");
                                // let results: Vec<HashMap<String, Value>> = bson::from_bson(bson::Bson::Array(execution_result)).unwrap();
                                //
                                // // let result: HashMap<String, Value> = bson::from_bson(bson::Bson::Document(person_document)).unwrap();
                                // // let result: HashMap<String, Value> = bson::from_bson(bson::Bson::Document(person_document)).unwrap();
                                // DataProviderResult::Multiple(results)
                                unimplemented!()
                            }
                        };

                        result
                    }
                    (Insert, single) => {
                        // let result: DataProviderResult = match single {
                        //     true => {
                        //         // let execution_result = collection.insert_one(document, mongo_options.insert_one()).expect("Document not found");
                        //         // let result: HashMap<String, Value> = bson::from_bson(bson::Bson::Document(execution_result)).unwrap();
                        //         // DataProviderResult::Single(result)
                        //         unimplemented!()
                        //     }
                        //     false => {
                        //         // let execution_result = collection.insert_many(document, mongo_options.insert_many()).expect("Document not found").unwrap();
                        //         // let result: HashMap<String, Value> = bson::from_bson(bson::Bson::Document(execution_result)).unwrap();
                        //         // DataProviderResult::Single(result)
                        //         unimplemented!()
                        //     }
                        // };
                        // result

                        unimplemented!()
                    }

                    _ => unimplemented!()
                }
            }
            _ => unimplemented!()
        };

        Ok(data_provider_result)
    }
}