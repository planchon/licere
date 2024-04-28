use crate::{errors::SchemaError, store::types::Tuple};
use tracing::{event, instrument, Level};

#[derive(Debug)]
pub struct Schema {
    pub entities: Vec<EntityDefinition>,
}

#[derive(Clone, Debug)]
pub struct EntityDefinition {
    pub name: String,
    pub relations: Vec<RelationDefinition>,
    pub permissions: Vec<PermissionDefinition>,
}

#[derive(Clone, Debug)]
pub struct RelationDefinition {
    pub name: String,
    pub entity_ref: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct PermissionDefinition {
    pub name: String,
    pub op: PermissionOperationDefinition,
}

#[derive(Clone, Debug)]
pub enum PermissionOperationDefinition {
    And(Vec<PermissionOperationDefinition>),
    Or(Vec<PermissionOperationDefinition>),
    Relation(String),
    UndirectRelation((String, String)),
}

impl Default for Schema {
    fn default() -> Self {
        Schema { entities: vec![] }
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
        }

        Ok(())
    }

    pub fn validate_tuple(&self, tuple: &Tuple) -> Result<(), SchemaError> {
        if let Err(e) = self.get_entity_definition(&tuple.entity) {
            return Err(e);
        }
        Ok(())
    }

    pub fn get_entity_definition(&self, entity: &String) -> Result<&EntityDefinition, SchemaError> {
        match self.entities.iter().find(|p| &p.name == entity) {
            Some(e) => Ok(e),
            None => Err(SchemaError::EntityNotExists(entity.clone())),
        }
    }
}

impl EntityDefinition {
    pub fn validate(&self, schema: &Schema) -> Result<(), SchemaError> {
        Ok(())
    }

    pub fn get_relation(&self, relation: &String) -> Result<&RelationDefinition, SchemaError> {
        match self.relations.iter().find(|p| &p.name == relation) {
            Some(r) => Ok(r),
            None => Err(SchemaError::EntityNotExists(relation.clone())),
        }
    }

    pub fn get_permission(
        &self,
        permission: &String,
    ) -> Result<&PermissionDefinition, SchemaError> {
        match self.permissions.iter().find(|p| &p.name == permission) {
            Some(p) => Ok(p),
            None => Err(SchemaError::EntityNotExists(permission.clone())),
        }
    }
}

impl PermissionDefinition {
    pub fn validate(&self, schema: &Schema) -> Result<(), SchemaError> {
        Ok(())
    }
}
