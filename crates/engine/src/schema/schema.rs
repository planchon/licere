use crate::{errors::SchemaError, store::types::Tuple};
use tracing::{event, instrument, Level};

pub struct Schema {
    pub entities: Vec<EntityDefinition>,
    pub entities_name: Vec<String>,
}

pub struct EntityDefinition {
    pub name: String,
    pub relations: Vec<RelationDefinition>,
    pub permissions: Vec<PermissionDefinition>,
}

pub struct RelationDefinition {
    pub name: String,
    pub entity_ref: Vec<String>,
}

pub struct PermissionDefinition {
    pub name: String,
    pub op: PermissionOperationDefinition,
}

pub enum PermissionOperationDefinition {
    And(Vec<PermissionOperationDefinition>),
    Or(Vec<PermissionOperationDefinition>),
    Entity(String),
    EntityWithPermission((String, String)),
}

impl Default for Schema {
    fn default() -> Self {
        Schema {
            entities: vec![],
            entities_name: vec![],
        }
    }
}

impl Schema {
    #[instrument(skip(self), name = "validate")]
    pub fn validate(&mut self) -> Result<(), SchemaError> {
        event!(Level::DEBUG, "validating the schema");
        // get all the valid entities
        for e in &self.entities {
            if let Err(err) = e.validate(&self) {
                return Err(err);
            }
            self.entities_name.push(e.name.clone());
        }

        Ok(())
    }

    pub fn validate_tuple(&self, tuple: &Tuple) -> Result<(), SchemaError> {
        if !self.entities_name.contains(&tuple.entity) {
            return Err(SchemaError::EntityNotExists(tuple.entity.clone()));
        }
        Ok(())
    }
}

impl EntityDefinition {
    pub fn validate(&self, schema: &Schema) -> Result<(), SchemaError> {
        Ok(())
    }
}

impl PermissionDefinition {
    pub fn validate(&self, schema: &Schema) -> Result<(), SchemaError> {
        Ok(())
    }
}
