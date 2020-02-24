extern crate json;

use std::collections::HashMap;

use self::json::JsonValue;

pub struct HttpContent {
    headers: HashMap<String, String>,
    body: String,
}

pub enum HttpMethods {
    GET,
    POST,
    PATCH,
    PUT,
    DELETE,
    HEAD,
    ALL,
}

pub struct Request {
    pub content: HttpContent,
    pub method: HttpMethods,
}

impl Request {
    pub fn get_content(&mut self) -> &mut HttpContent {
        &mut self.content
    }
}

pub type Response = HttpContent;

enum HandlerErrors {
    ValidationException {
        message: String
    },
    IllegalArgument {},
}

impl HttpContent {
    pub fn new() -> HttpContent {
        HttpContent { headers: HashMap::new(), body: "{}".into() }
    }

    pub fn with_header(&mut self, key: &String, value: &String) -> &mut Self {
        let key = key.clone();
        let value = value.clone();
        self.headers.insert(key, value);
        self
    }

    pub fn drop_header(&mut self, key: &String) -> &mut Self {
        self.headers.remove(key.clone().as_str());
        self
    }

    pub fn get_header(&self, key: &String) -> &String {
        self.headers.get(key.as_str()).unwrap()
    }
    pub fn with_body(&mut self, body: String) -> &mut Self {
        self.body = body;
        &mut *self
    }
    pub fn get_body(&self) -> &String {
        &self.body
    }
    pub fn with_body_json(&mut self, json: JsonValue) -> &mut Self {
        self.body = json.dump();
        &mut *self
    }

    pub fn json_add(&mut self, key: &str, value: &JsonValue) -> &mut Self {
        let mut json = self.get_body_json();
        json.insert(key, value.clone());
        self.with_body_json(json)
    }

    pub fn json_remove(&mut self, key: &str) -> &mut Self {
        let json = self.get_body_json()
            .remove(key);
        self.with_body_json(json)
    }

    pub fn get_body_json(&self) -> JsonValue {
        json::parse(&self.body.as_str()).expect("can't read json from the body")
    }
}
