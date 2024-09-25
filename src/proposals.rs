use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use crate::pb::cosmos::upgrade::v1beta1::{MsgSoftwareUpgrade, SoftwareUpgradeProposal};

pub fn insert_message_software_upgrade(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if let Ok(msg_software_upgrade) = <MsgSoftwareUpgrade as prost::Message>::decode(content.value.as_slice()) {
            let proposer = msg_submit_proposal.proposer.as_str();
            let initial_deposit = msg_submit_proposal.initial_deposit.get(0).unwrap();
            let initial_deposit_denom = initial_deposit.denom.as_str();
            let initial_deposit_amount = initial_deposit.amount.as_str();

            let authority = msg_software_upgrade.authority.as_str();

            let title = msg_submit_proposal.title.as_deref().unwrap_or("");
            let summary = msg_submit_proposal.summary.as_deref().unwrap_or("");
            let metadata = msg_submit_proposal.metadata.as_deref().unwrap_or("");

            let plan = msg_software_upgrade.plan.unwrap();
            let plan_name = plan.name.as_str();
            let plan_height = plan.height;
            let plan_info = plan.info.as_str();

            // There can be multiple submit_proposal events in a single tx
            // So we need to filter the events and get the proposal_id from the correct one
            let proposal_id = tx_result
                .events
                .iter()
                .filter(|event| event.r#type == "submit_proposal") // filter to get all submit_proposal events
                .flat_map(|event| event.attributes.iter()) // flatten all attributes
                .find(|attr| attr.key == "proposal_id") // find the one with the proposal_id attribute
                .and_then(|attr| attr.value.parse::<u64>().ok()) // parse it as u64 if found
                .expect(&format!(
                    "proposal_id not found for message software upgrade at block {}",
                    clock.number
                ));

            // Create Proposal entity
            tables
                .create_row("Proposal", proposal_id.to_string().as_str())
                .set("id", proposal_id.to_string().as_str())
                .set("txHash", tx_hash)
                .set("blockNumber", clock.number)
                .set("type", "software_upgrade_proposal")
                .set("proposer", proposer)
                .set("initialDepositDenom", initial_deposit_denom)
                .set("initialDepositAmount", initial_deposit_amount)
                .set("authority", authority)
                .set("title", title)
                .set("description", summary)
                .set("metadata", metadata);

            // Create SoftwareUpgradeProposal entity
            tables
                .create_row("SoftwareUpgradeProposal", proposal_id.to_string().as_str())
                .set("id", proposal_id.to_string().as_str())
                .set("planName", plan_name)
                .set("planHeight", plan_height)
                .set("planInfo", plan_info)
                .set("proposal", proposal_id.to_string().as_str());
        }
    }
}

pub fn insert_software_upgrade_proposal(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if let Ok(software_upgrade_proposal) =
            <SoftwareUpgradeProposal as prost::Message>::decode(content.value.as_slice())
        {
            let proposer = msg_submit_proposal.proposer.as_str();
            let initial_deposit = msg_submit_proposal.initial_deposit.get(0).unwrap();
            let initial_deposit_denom = initial_deposit.denom.as_str();
            let initial_deposit_amount = initial_deposit.amount.as_str();

            let authority = tx_result
                .events
                .iter()
                .find(|event| event.r#type == "coin_received")
                .and_then(|event| event.attributes.iter().find(|attr| attr.key == "receiver"))
                .map(|attr| attr.value.as_str())
                .unwrap_or("");

            let title = software_upgrade_proposal.title.as_str();
            let description = software_upgrade_proposal.description.as_str();

            let plan = software_upgrade_proposal.plan.unwrap();
            let plan_name = plan.name.as_str();
            let plan_height = plan.height;
            let plan_info = plan.info.as_str();

            let proposal_id = tx_result
                .events
                .iter()
                .filter(|event| event.r#type == "submit_proposal") // filter to get all submit_proposal events
                .flat_map(|event| event.attributes.iter()) // flatten all attributes
                .find(|attr| attr.key == "proposal_id") // find the one with the proposal_id attribute
                .and_then(|attr| attr.value.parse::<u64>().ok()) // parse it as u64 if found
                .expect(&format!(
                    "proposal_id not found for software upgrade proposal at block {}",
                    clock.number
                ));

            // Create Proposal entity
            tables
                .create_row("Proposal", proposal_id.to_string().as_str())
                .set("id", proposal_id.to_string().as_str())
                .set("txHash", tx_hash)
                .set("blockNumber", clock.number)
                .set("type", "software_upgrade_proposal")
                .set("proposer", proposer)
                .set("initialDepositDenom", initial_deposit_denom)
                .set("initialDepositAmount", initial_deposit_amount)
                .set("authority", authority)
                .set("title", title)
                .set("description", description)
                .set("metadata", "");

            // Create SoftwareUpgradeProposal entity
            tables
                .create_row("SoftwareUpgradeProposal", proposal_id.to_string().as_str())
                .set("id", proposal_id.to_string().as_str())
                .set("planName", plan_name)
                .set("planHeight", plan_height)
                .set("planInfo", plan_info)
                .set("proposal", proposal_id.to_string().as_str());
        }
    }
}
