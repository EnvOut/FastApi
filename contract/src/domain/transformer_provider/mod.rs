use std::collections::HashMap;

pub mod chain_transformer;

pub trait TransformerProvider {}

pub struct TransformerProviderRegistry {
    key_value_idx: HashMap<String, Box<dyn TransformerProvider>>,
}

impl TransformerProviderRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }
}