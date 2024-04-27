use crate::errors::SchemaError;
use crate::schema::schema::Schema;
use crate::store::types::Tuple;

use super::types::Store;

pub struct MemoryStore<'a> {
    data: Vec<Tuple>,
    schema: &'a Schema,
}

impl<'a> MemoryStore<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        MemoryStore {
            data: vec![],
            schema,
        }
    }
}

impl<'a> Store for MemoryStore<'a> {
    fn add(&mut self, t: Tuple) -> Result<(), SchemaError> {
        if let Err(e) = self.schema.validate_tuple(&t) {
            return Err(e);
        }
        self.data.push(t);
        Ok(())
    }
}
