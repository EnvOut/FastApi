use std::collections::HashMap;
use std::iter::Map;
use std::ops::Index;

//    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Endpoint {}

//    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EndpointRegistry {
    key_value_idx: HashMap<String, Endpoint>,
}

impl Index<String> for EndpointRegistry {
    type Output = Endpoint;

    #[inline]
    fn index(&self, index: String) -> &Self::Output {
        &self.key_value_idx[&index]
    }
}

impl EndpointRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }
}