use prost_types::Any;
use substreams_entity_change::tables::Tables;

use crate::pb::cosmos::upgrade::v1beta1::{MsgSoftwareUpgrade, Plan, SoftwareUpgradeProposal};

pub fn create_row_for_upgrade(tables: &mut Tables, proposal_id: &str, plan: Plan) {
    tables
        .create_row("SoftwareUpgradeProposal", &proposal_id)
        .set("name", plan.name.as_str())
        .set("height", plan.height)
        .set("info", plan.info.as_str())
        .set("proposal", proposal_id);
}

pub fn create_software_upgrade(tables: &mut Tables, content: &Any, proposal_id: &str) {
    if let Ok(msg) = <MsgSoftwareUpgrade as prost::Message>::decode(content.value.as_slice()) {
        let plan = msg.plan.expect("missing software upgrade plan");
        create_row_for_upgrade(tables, proposal_id, plan);
    }

    // Deprecated: This legacy proposal is deprecated in favor of Msg-based gov
    if let Ok(msg) = <SoftwareUpgradeProposal as prost::Message>::decode(content.value.as_slice()) {
        let plan = msg.plan.expect("missing software upgrade plan");
        create_row_for_upgrade(tables, proposal_id, plan);
    }
}
