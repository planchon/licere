use cli::seed::operation::{create_schema, seeding_store};
use engine::{
    engine::engine::{Engine, SlowEngine},
    store::{memory::MemoryStore, types::Tuple},
};
use tracing::{event, subscriber, Level};
use tracing_subscriber::FmtSubscriber;

fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("Error while setting the tracing global subscriber");

    let mut schema = create_schema();
    event!(Level::DEBUG, "schema created");

    if let Err(e) = schema.validate() {
        panic!("Error in the schema : {}", e);
    }

    let mut memory_store = MemoryStore::new();
    seeding_store(&mut memory_store);

    let engine = SlowEngine::new(memory_store, schema);

    let res = engine.check(Tuple {
        entity: "user".into(),
        entity_id: "paul".into(),
        relation: "read".into(),
        subject: "folder".into(),
        subject_id: "ppl".into(),
    });

    event!(Level::INFO, "Launching the Licere CLI");
}
