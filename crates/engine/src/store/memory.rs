use crate::errors::StoreError;
use crate::store::types::Tuple;

use super::types::Store;

pub struct MemoryStore {
    data: Vec<Tuple>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore { data: vec![] }
    }
}

impl Store for MemoryStore {
    fn add(&mut self, t: Tuple) -> Result<(), StoreError> {
        self.data.push(t);
        Ok(())
    }
}
