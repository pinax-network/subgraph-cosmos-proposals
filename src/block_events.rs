use substreams_cosmos::Block;
use substreams_entity_change::tables::Tables;

use crate::utils::get_attribute_value;

pub fn push_block_events(block: &Block, tables: &mut Tables) {
    for event in block.events.iter() {
        if event.r#type == "active_proposal" {
            if let (Some(proposal_id), Some(status)) = (
                get_attribute_value(event, "proposal_id"),
                get_attribute_value(event, "proposal_result"),
            ) {
                tables.update_row("Proposal", proposal_id).set("status", status);
            }
        }
    }
}
