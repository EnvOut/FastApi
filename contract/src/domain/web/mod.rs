//#[macro_use]
extern crate json;

use std::collections::HashMap;

use self::json::JsonValue;

//pub trait Request where Self: Sized+Copy+Clone {
pub trait Request {
    fn set_header(&self, key: &String, value: &String) -> Self;
    fn get_header(&self, key: &String) -> &String;
    fn set_body(&self, body: String) -> Self;
    fn get_body(&self) -> &'static String;
    fn set_body_json(&self, json: JsonValue) -> Self;
    fn get_body_json(&self) -> JsonValue;
}