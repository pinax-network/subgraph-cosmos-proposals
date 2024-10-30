use crate::{
    block_events::push_block_events, blocks::create_block, genesis_params::push_genesis_params,
    transactions::push_transactions,
};
use substreams::store::{StoreGet, StoreGetArray, StoreGetString};
use substreams::{errors::Error, pb::substreams::Clock};
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(
    params: String,
    clock: Clock,
    block: Block,
    gov_params: StoreGetString,
) -> Result<EntityChanges, Error> {
    let min_deposit = gov_params.get_first("min_deposit");
    let max_deposit_period = gov_params.get_first("max_deposit_period");
    let voting_period = gov_params.get_first("voting_period");
    let quorum = gov_params.get_first("quorum");
    let threshold = gov_params.get_first("threshold");
    let veto_threshold = gov_params.get_first("veto_threshold");

    substreams::log::debug!("min_deposit: {:?}", min_deposit);
    substreams::log::debug!("max_deposit_period: {:?}", max_deposit_period);
    substreams::log::debug!("voting_period: {:?}", voting_period);
    substreams::log::debug!("quorum: {:?}", quorum);
    substreams::log::debug!("threshold: {:?}", threshold);
    substreams::log::debug!("veto_threshold: {:?}", veto_threshold);
    substreams::log::debug!("test: {:?}", gov_params.get_first("test"));

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
