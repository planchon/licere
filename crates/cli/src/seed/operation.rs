use engine::{
    schema::schema::{
        EntityDefinition, PermissionDefinition, PermissionOperationDefinition, RelationDefinition,
        Schema,
    },
    store::{
        memory::MemoryStore,
        types::{Store, Tuple},
    },
};
use tracing::{event, Level};

pub fn seeding_store(store: &mut MemoryStore) {
    event!(Level::INFO, "starting the seeding operation");

    let _ = store.add(Tuple::new("folder", "root", "admin", "user", "admin"));
    _ = store.add(Tuple::new("folder", "ppl", "parent", "folder", "root"));
    _ = store.add(Tuple::new("folder", "root", "reader", "user", "paul"));
    _ = store.add(Tuple::new("folder", "ppl", "admin", "user", "paul"));
}

pub fn create_schema() -> Schema {
    Schema {
        entities: vec![
            EntityDefinition {
                name: "user".to_string(),
                relations: vec![],
                permissions: vec![],
            },
            EntityDefinition {
                name: "folder".to_string(),
                relations: vec![
                    RelationDefinition {
                        name: "parent".into(),
                        entity_ref: vec!["folder".into()],
                    },
                    RelationDefinition {
                        name: "reader".into(),
                        entity_ref: vec!["user".into()],
                    },
                    RelationDefinition {
                        name: "admin".into(),
                        entity_ref: vec!["user".into()],
                    },
                ],
                permissions: vec![
                    PermissionDefinition {
                        name: "read".into(),
                        op: PermissionOperationDefinition::Or(vec![
                            PermissionOperationDefinition::Relation("reader".into()),
                            PermissionOperationDefinition::Relation("admin".into()),
                            PermissionOperationDefinition::UndirectRelation((
                                "parent".into(),
                                "read".into(),
                            )),
                        ]),
                    },
                    PermissionDefinition {
                        name: "edit".into(),
                        op: PermissionOperationDefinition::Or(vec![
                            PermissionOperationDefinition::Relation("admin".into()),
                            PermissionOperationDefinition::UndirectRelation((
                                "parent".into(),
                                "edit".into(),
                            )),
                        ]),
                    },
                ],
            },
        ],
        ..Default::default()
    }
}
