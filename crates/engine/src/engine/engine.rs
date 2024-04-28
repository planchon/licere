use std::fmt::Error;

use tracing::{event, Level};

use crate::{
    errors::EngineError,
    schema::schema::{PermissionOperationDefinition, Schema},
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

    fn flat_permission(
        &self,
        mut flat: &mut Vec<(String, String)>,
        entity: &String,
        p: &PermissionOperationDefinition,
    ) {
        match p {
            PermissionOperationDefinition::And(v) => {
                v.iter()
                    .for_each(|op| self.flat_permission(&mut flat, entity, op));
            }
            PermissionOperationDefinition::Or(v) => {
                v.iter()
                    .for_each(|op| self.flat_permission(&mut flat, entity, op));
            }
            PermissionOperationDefinition::Relation(r) => flat.push((entity.clone(), r.clone())),
            PermissionOperationDefinition::UndirectRelation((a, b)) => {
                flat.push((a.clone(), b.clone()))
            }
        }
    }
}

impl Engine for SlowEngine {
    fn check(&self, t: Tuple) -> Result<CheckResult, EngineError> {
        let entity_ref_res = self.schema.get_entity_definition(&t.subject);
        if entity_ref_res.is_err() {
            let ent = t.entity;
            event!(Level::ERROR, "Entity {} not found", ent);
            return Err(EngineError::EntityNotFound(ent));
        }

        let entity_ref = entity_ref_res.unwrap();

        let permission_ref_res = entity_ref.get_permission(&t.relation);
        if permission_ref_res.is_err() {
            let rel = t.relation;
            event!(Level::ERROR, "Relation {} not found", rel);
            return Err(EngineError::RelationNotFound(rel));
        }

        let permission_ref = permission_ref_res.unwrap();
        let permission_op = &permission_ref.op;

        let mut flat_relation = vec![];
        self.flat_permission(&mut flat_relation, &t.entity.clone(), permission_op);

        println!("flat {:?}", flat_relation);

        Ok(CheckResult::DENIED)
    }

    fn write(&self, t: Tuple) -> Result<(), EngineError> {
        Ok(())
    }
}
