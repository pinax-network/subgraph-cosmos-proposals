use prost_types::Any;
use substreams_entity_change::tables::Tables;

use crate::pb::cosmos::upgrade::v1beta1::{MsgSoftwareUpgrade, SoftwareUpgradeProposal};

pub fn create_software_upgrade(tables: &mut Tables, content: &Any, proposal_id: &str) {
    if let Ok(msg_software_upgrade) = <MsgSoftwareUpgrade as prost::Message>::decode(content.value.as_slice()) {
        let plan = msg_software_upgrade.plan.expect("missing software upgrade plan");
        tables
            .create_row("SoftwareUpgradeProposal", &proposal_id)
            .set("planName", plan.name.as_str())
            .set("planHeight", plan.height)
            .set("planInfo", plan.info.as_str())
            .set("proposal", proposal_id);
    }
    // Deprecated: This legacy proposal is deprecated in favor of Msg-based gov
    if let Ok(msg_software_upgrade) = <SoftwareUpgradeProposal as prost::Message>::decode(content.value.as_slice()) {
        let plan = msg_software_upgrade.plan.expect("missing software upgrade plan");
        tables
            .create_row("SoftwareUpgradeProposal", &proposal_id)
            .set("planName", plan.name.as_str())
            .set("planHeight", plan.height)
            .set("planInfo", plan.info.as_str())
            .set("proposal", proposal_id);
    }
}
