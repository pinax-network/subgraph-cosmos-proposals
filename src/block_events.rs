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
        let frm_status = format_proposal_status(&status);
        // tables.update_row("Proposal", proposal_id).set("status", frm_status);
    }
}

fn is_proposal_event(event: &substreams_cosmos::pb::Event) -> bool {
    matches!(
        event.r#type.as_str(),
        "active_proposal" | "inactive_proposal" | "cancel_proposal"
    )
}

fn format_proposal_status(status: &str) -> String {
    match status {
        "proposal_passed" => "Passed".to_string(),
        "proposal_rejected" => "Rejected".to_string(),
        "proposal_dropped" => "Dropped".to_string(),
        "proposal_failed" => "Failed".to_string(),
        "proposal_canceled" => "Canceled".to_string(),
        "expedited_proposal_rejected" => "Rejected".to_string(),
        "optimistic_proposal_rejected" => "Rejected".to_string(),
        _ => status.to_string(),
    }
}
