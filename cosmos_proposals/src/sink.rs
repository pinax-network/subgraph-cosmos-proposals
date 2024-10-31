use crate::utils::extract_gov_params;
use crate::{
    block_events::push_block_events, blocks::create_block, genesis_params::push_gov_params,
    transactions::push_transactions,
};
use substreams::store::{StoreGet, StoreGetString};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Block, gov_params_store: StoreGetString) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();

    let gov_params = extract_gov_params(&gov_params_store);

    push_gov_params(&mut tables, &clock, &gov_params);
    push_transactions(&block, &mut tables, &clock, &gov_params);
    push_block_events(&block, &mut tables);

    // only emit block Entity if previous entities were emitted
    if tables.tables.len() != 0 {
        create_block(&mut tables, &clock);
    }

    Ok(tables.to_entity_changes())
}
