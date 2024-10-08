use substreams::{log, pb::substreams::Clock};
use substreams_entity_change::tables::Tables;

use crate::serde_genesis::GenesisParams;

pub fn push_genesis_params(params: &String, clock: &Clock, tables: &mut Tables) {
    let parsed: GenesisParams = serde_json::from_str(&params).expect("Failed to parse params");
    if clock.number == 1 {
        log::debug!("GenesisParams: {:?}", parsed);
    }
}
