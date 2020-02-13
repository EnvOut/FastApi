use std::sync::Arc;

use crate::domain::web::Request;
use json::JsonValue;

//pub struct RequestTransformer {
//    pub lambda: Box<dyn Fn(&mut Request) -> &mut Request>
//}
//
//impl RequestTransformer {
//    pub fn transform<'a>(&self, request: &'a mut Request) -> &'a Request {
//        (self.lambda)(request)
//    }
//
//    fn new(lambda: Box<dyn Fn(&mut Request) -> &mut Request>) -> Self {
//        Self { lambda }
//    }
//}

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
            RequestTransformers::HeaderAdd(name, value) => Box::new(move |req: &mut Request| req.with_header(name, value)),
            RequestTransformers::HeaderRemove(name) => Box::new(move |req: &mut Request| req.drop_header(name)),
            RequestTransformers::JsonAdd(property, value) => Box::new(move |req: &mut Request| req.json_add(property, value)),
            RequestTransformers::JsonRemove(property) => Box::new(move |req: &mut Request| req.json_remove(property))
        };
        lambda(request)
    }
}

pub type RequestTransformChain = ChainTransformer<RequestTransformers>;