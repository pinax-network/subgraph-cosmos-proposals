use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

use crate::order_by::insert_order_by;

pub fn insert_proposal_entity(
    tables: &mut Tables,
    id: &str,
    tx_hash: &str,
    clock: &Clock,
    proposal_type: &str,
    proposer: &str,
    authority: &str,
    title: &str,
    description: &str,
    metadata: &str,
) {
    let row = tables
        .create_row("Proposal", id)
        .set("txHash", tx_hash)
        .set("type", proposal_type)
        .set("proposer", proposer)
        .set("authority", authority)
        .set("title", title)
        .set("description", description)
        .set("metadata", metadata);

    insert_order_by(row, clock);
}
