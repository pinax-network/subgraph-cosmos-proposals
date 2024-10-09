use substreams_cosmos::Block;
use substreams_entity_change::tables::Tables;

use crate::utils::get_attribute_value;

pub fn push_block_events(block: &Block, tables: &mut Tables) {
    for event in block.events.iter() {
        if is_proposal_event(event) {
            update_proposal_status(event, tables);
        }
    }
}

fn update_proposal_status(event: &substreams_cosmos::pb::Event, tables: &mut Tables) {
    if let (Some(proposal_id), Some(status)) = (
        get_attribute_value(event, "proposal_id"),
        get_attribute_value(event, "proposal_result"),
    ) {
        tables.update_row("Proposal", proposal_id).set("status", status);
    }
}

fn is_proposal_event(event: &substreams_cosmos::pb::Event) -> bool {
    event.r#type == "active_proposal" || event.r#type == "inactive_proposal"
}
