use crate::errors::{SchemaError, StoreError};

#[derive(Debug, Clone)]
pub struct Tuple {
    pub entity: String,
    pub entity_id: String,
    pub relation: String,
    pub subject: String,
    pub subject_id: String,
}

impl Tuple {
    pub fn new(
        entity: &str,
        entity_id: &str,
        relation: &str,
        subject: &str,
        subject_id: &str,
    ) -> Self {
        Tuple {
            entity: entity.to_string(),
            entity_id: entity_id.to_string(),
            relation: relation.to_string(),
            subject: subject.to_string(),
            subject_id: subject_id.to_string(),
        }
    }
}

pub trait Store {
    fn add(&mut self, t: Tuple) -> Result<(), StoreError>;
}
