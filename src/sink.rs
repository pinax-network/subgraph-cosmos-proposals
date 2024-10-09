use crate::{genesis_params::push_genesis_params, transactions::push_transactions};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    // push_genesis_params(&params, &clock, &mut tables);
    push_transactions(&block, &mut tables, &clock);

    Ok(tables.to_entity_changes())
}
