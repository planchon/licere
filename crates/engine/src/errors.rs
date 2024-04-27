use core::fmt;
use std::error;

#[derive(Debug)]
pub enum StoreError {
    TupleNotFound(String),
}

pub enum SchemaError {
    BadEntityRelationDefinition(String),
    EntityNotExists(String),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StoreError::TupleNotFound(ref message) => {
                write!(f, "Tuple is not found in the store : {}", message)
            }
        }
    }
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SchemaError::BadEntityRelationDefinition(ref ent) => {
                write!(f, "Poor entity definition for entity {}", ent)
            }
            SchemaError::EntityNotExists(ref ent) => {
                write!(f, "Entity {} does not exists", ent)
            }
        }
    }
}

impl error::Error for StoreError {}
