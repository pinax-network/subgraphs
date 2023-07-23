use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables;

#[substreams::handlers::map]
pub fn graph_out(clock: Clock) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let timestamp = clock.timestamp.unwrap();

    tables
        .create_row("Clock", clock.id)
        .set_bigint("number", &clock.number.to_string())
        .set_bigint("seconds", &timestamp.seconds.to_string())
        .set_bigint("nanos", &timestamp.nanos.to_string())
        .set("timestamp", timestamp.to_string());

    Ok(tables.to_entity_changes())
}