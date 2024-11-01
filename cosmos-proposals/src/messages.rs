use crate::{proposals::proposals::handle_proposals, utils::GovernanceParamsFlat};
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

pub fn push_messages(
    tables: &mut Tables,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
    messages: &[prost_types::Any],
    gov_params: &GovernanceParamsFlat,
) {
    for message in messages.iter() {
        handle_proposals(tables, clock, message, tx_result, tx_hash, gov_params);
    }
}
