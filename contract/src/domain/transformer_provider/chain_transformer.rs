use std::sync::Arc;

use json::JsonValue;

use crate::domain::web::Request;

pub struct ChainTransformer<T> {
    chain: Vec<Arc<T>>
}

impl<T> ChainTransformer<T> {
    pub fn with_transformer(&mut self, transformer: T) -> &mut Self {
        self.chain.push(Arc::new(transformer));
        self
    }

    pub fn new() -> ChainTransformer<T> {
        ChainTransformer { chain: vec![] }
    }

    pub fn get_transformers(&self) -> Vec<Arc<T>> {
        self.chain.clone()
    }
}

impl ChainTransformer<Request> {
    pub fn new_request_transformer_chain() -> ChainTransformer<Request> { ChainTransformer::new() }
}

#[derive(Eq, PartialEq, Debug)]
pub enum RequestTransformers {
    HeaderAdd(String, String),
    HeaderRemove(String),
    JsonAdd(String, JsonValue),
    JsonRemove(String),
}

impl RequestTransformers {
    pub fn transform<'a>(&self, request: &'a mut Request) -> &'a Request {
        let lambda: Box<dyn Fn(&mut Request) -> &mut Request> = match self {
            RequestTransformers::HeaderAdd(name, value) => Box::new(move |req: &mut Request| {
                req.get_content().with_header(name, value);
                req
            }),
            RequestTransformers::HeaderRemove(name) => Box::new(move |req: &mut Request| {
                req.get_content().drop_header(name);
                req
            }),
            RequestTransformers::JsonAdd(property, value) => Box::new(move |req: &mut Request| {
                req.get_content().json_add(property, value);
                req
            }),
            RequestTransformers::JsonRemove(property) => Box::new(move |req: &mut Request| {
                req.get_content().json_remove(property);
                req
            }),
        };
        lambda(request)
    }
}

pub type RequestTransformChain = ChainTransformer<RequestTransformers>;