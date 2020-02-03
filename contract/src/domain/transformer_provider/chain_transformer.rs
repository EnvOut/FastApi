use crate::domain::web::Request;

pub struct Transformer{
    // problem
    lambda: fn(Box<dyn Request>) -> Box<dyn Request>,
}

impl Transformer {
    pub fn transform<R:Request>(&self, request: Box<R>) -> Box<R> {
//    pub fn transform(&self, request: Box<dyn Request>) -> Box<dyn Request> {
//        self.lambda(request)
        unimplemented!()
    }

    pub fn add_header(key: &String, value: &String) -> Transformer {
//        let lambda: fn(Box<dyn Request>) -> Box<dyn Request> = |request| {
////            let mut request = &r;
////            request.set_header("","");
////            request.set
//
//            request.get_body();
//            unimplemented!()
//        };
//
//        Transformer { lambda };

        unimplemented!()
    }
}

pub struct ChainTransformer {
    chain: Vec<Transformer>
}