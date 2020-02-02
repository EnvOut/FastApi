use std::collections::HashMap;

pub trait TransformerProvider {}

pub struct TransformerProviderRegistry {
    key_value_idx: HashMap<String, Box<dyn TransformerProvider>>,
}

impl TransformerProviderRegistry {
    pub fn is_empty(&self) -> bool {
        self.key_value_idx.is_empty()
    }
}