use std::collections::HashMap;
use std::ops::Index;

use crate::domain::web::{Request, Response};

// pub mod crud_endpoint;

//    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct EndpointProvider {}
pub trait AbstractEndpointProvider {
    fn handle(&mut self, req: &Request) -> &Response;
}

//    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndpointRegistry {
    key_value_idx: HashMap<String, Box<dyn AbstractEndpointProvider>>,
}

// impl Index<String> for EndpointRegistry {
//     type Output = AbstractEndpointProvider;
//
//     #[inline]
//     fn index(&self, index: String) -> &Self::Output {
//         &self.key_value_idx[&index]
//     }
// }

impl EndpointRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }
}