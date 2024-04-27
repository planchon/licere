use cli::seed::operation::{create_schema, seeding_store};
use engine::store::memory::MemoryStore;
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

    let mut memory_store = MemoryStore::new(&schema);

    seeding_store(&mut memory_store);

    event!(Level::INFO, "Launching the Licere CLI");
}
