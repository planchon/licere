use crate::{errors::SchemaError, store::types::Tuple};
use tracing::{event, instrument, Level};

pub struct Schema {
    pub entities: Vec<EntityDefinition>,
    pub entities_name: Vec<String>,
}

#[derive(Clone)]
pub struct EntityDefinition {
    pub name: String,
    pub relations: Vec<RelationDefinition>,
    pub permissions: Vec<PermissionDefinition>,
}

#[derive(Clone)]
pub struct RelationDefinition {
    pub name: String,
    pub entity_ref: Vec<String>,
}

#[derive(Clone)]
pub struct PermissionDefinition {
    pub name: String,
    pub op: PermissionOperationDefinition,
}

#[derive(Clone)]
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

    pub fn get_entity_definition(&self, entity: String) -> Result<&EntityDefinition, SchemaError> {
        match self
            .entities
            .binary_search_by(|probe| probe.name.cmp(&entity))
        {
            Ok(idx) => Ok(&self.entities[idx]),
            Err(_) => Err(SchemaError::EntityNotExists(entity)),
        }
    }
}

impl EntityDefinition {
    pub fn validate(&self, schema: &Schema) -> Result<(), SchemaError> {
        Ok(())
    }

    pub fn get_relation(&self, relation: String) -> Result<&RelationDefinition, SchemaError> {
        match self
            .relations
            .binary_search_by(|probe| probe.name.cmp(&relation))
        {
            Ok(idx) => Ok(&self.relations[idx]),
            Err(_) => Err(SchemaError::EntityNotExists(relation)),
        }
    }
}

impl PermissionDefinition {
    pub fn validate(&self, schema: &Schema) -> Result<(), SchemaError> {
        Ok(())
    }
}
