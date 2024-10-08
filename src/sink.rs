use core::panic;

use crate::{serde_genesis::GenesisParams, transactions::push_transactions};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    // if params.is_empty() {
    //     panic!("GenesisParams is not yet implemented");
    // }

    // let parsed: GenesisParams = serde_json::from_str(&params).expect("Failed to parse params");

    let mut tables = Tables::new();
    push_transactions(&block, &mut tables, &clock);

    Ok(tables.to_entity_changes())
}
