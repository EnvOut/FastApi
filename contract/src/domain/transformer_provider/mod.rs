use std::collections::HashMap;

use crate::domain::transformer_provider::chain_transformer::RequestTransformChain;

pub mod chain_transformer;

pub trait TransformerProvider {}

pub struct TransformerProviderRegistry {
    key_value_idx: HashMap<String, RequestTransformChain>,
}

impl TransformerProviderRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }

    pub fn new() -> TransformerProviderRegistry {
        TransformerProviderRegistry { key_value_idx: HashMap::new() }
    }

    pub fn add(&mut self, name: &String, value: RequestTransformChain) -> &mut Self {
        self.key_value_idx.insert(name.clone(), value);
        self
    }
}