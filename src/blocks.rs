use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

use crate::utils::to_date;

pub fn create_block(tables: &mut Tables, clock: &Clock) {
    // timestamp
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    let seconds = timestamp.seconds;
    let date = to_date(clock);
    let block_number = clock.number.to_string();

    tables
        .create_row("Block", clock.id.as_str())
        .set_bigint("number", &block_number.to_string())
        .set_bigint("timestamp", &seconds.to_string())
        .set("date", date);
}
