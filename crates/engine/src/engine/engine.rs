use crate::{
    errors::EngineError,
    schema::schema::Schema,
    store::{memory::MemoryStore, types::Tuple},
};

pub struct SlowEngine {
    pub store: MemoryStore,
    pub schema: Schema,
}

pub enum CheckResult {
    OK,
    DENIED,
}

pub trait Engine {
    fn check(&self, t: Tuple) -> Result<CheckResult, EngineError>;
    fn write(&self, t: Tuple) -> Result<(), EngineError>;
}

impl SlowEngine {
    pub fn new(store: MemoryStore, schema: Schema) -> Self {
        SlowEngine { store, schema }
    }
}

impl Engine for SlowEngine {
    fn check(&self, t: Tuple) -> Result<CheckResult, EngineError> {
        Ok(CheckResult::DENIED)
    }

    fn write(&self, t: Tuple) -> Result<(), EngineError> {
        Ok(())
    }
}
