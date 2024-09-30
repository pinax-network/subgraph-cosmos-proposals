use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

pub fn insert_block(tables: &mut Tables, clock: &Clock) {
    tables
        .create_row("Block", &clock.id.to_string())
        .set_bigint("number", &clock.number.to_string())
        .set("timestamp", clock.timestamp.as_ref().expect("timestamp missing"));
}
