use crate::{
    block_events::push_block_events, blocks::create_block, genesis_params::push_genesis_params,
    transactions::push_transactions,
};
use substreams::store::{StoreGet, StoreGetArray};
use substreams::{errors::Error, pb::substreams::Clock, skip_empty_output, store::StoreGetString};
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(
    params: String,
    clock: Clock,
    block: Block,
    store: StoreGetArray<String>,
) -> Result<EntityChanges, Error> {
    let proposals_passed = store.get_last("passed_proposal_ids");
    let genesis_params = store.get_last("genesis_params");
    let gov_param_proposals = store.get_last("gov_param_proposals");

    // causes the module output to be skipped if it has zero bytes
    skip_empty_output();

    let mut tables = Tables::new();

    push_genesis_params(&mut tables, &clock, &params);
    push_transactions(&block, &mut tables, &clock);
    push_block_events(&block, &mut tables);

    // only emit block Entity if previous entities were emitted
    if tables.tables.len() != 0 {
        create_block(&mut tables, &clock);
    }

    Ok(tables.to_entity_changes())
}
