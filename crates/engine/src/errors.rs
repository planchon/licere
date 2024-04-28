use core::fmt;
use std::error;

#[derive(Debug)]
pub enum StoreError {
    TupleNotFound(String),
}

#[derive(Debug)]
pub enum SchemaError {
    BadEntityRelationDefinition(String),
    EntityNotExists(String),
    RelationNotExists(String),
}

#[derive(Debug)]
pub enum EngineError {
    GeneralEngineError,
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
            SchemaError::RelationNotExists(ref rel) => {
                write!(f, "Relation {} does not exists", rel)
            }
        }
    }
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            EngineError::GeneralEngineError => {
                write!(f, "General engine error")
            }
        }
    }
}

impl error::Error for StoreError {}
impl error::Error for SchemaError {}
impl error::Error for EngineError {}
