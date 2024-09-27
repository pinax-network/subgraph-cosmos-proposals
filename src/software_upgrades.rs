use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use crate::pb::cosmos::upgrade::v1beta1::{MsgSoftwareUpgrade, SoftwareUpgradeProposal};
use crate::utils::{extract_authority, extract_initial_deposit, extract_proposal_id};

pub fn insert_message_software_upgrade(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(msg_software_upgrade) = <MsgSoftwareUpgrade as prost::Message>::decode(content.value.as_slice()) {
        let proposer = msg.proposer.as_str();
        let (initial_deposit_denom, initial_deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

        let authority = msg_software_upgrade.authority.as_str();

        let title = msg.title.as_str();
        let summary = msg.summary.as_str();
        let metadata = msg.metadata.as_str();

        let plan = msg_software_upgrade.plan.unwrap();
        let plan_name = plan.name.as_str();
        let plan_height = plan.height;
        let plan_info = plan.info.as_str();

        // There can be multiple submit_proposal events in a single tx
        // So we need to filter the events and get the proposal_id from the correct one
        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        // Create Proposal entity
        tables
            .create_row("Proposal", &proposal_id)
            .set("id", &proposal_id)
            .set("txHash", tx_hash)
            .set("blockNumber", clock.number)
            .set("type", "SoftwareUpgrade")
            .set("proposer", proposer)
            .set("initialDepositDenom", initial_deposit_denom)
            .set("initialDepositAmount", initial_deposit_amount)
            .set("authority", authority)
            .set("title", title)
            .set("description", summary)
            .set("metadata", metadata);

        // Create SoftwareUpgradeProposal entity
        tables
            .create_row("SoftwareUpgradeProposal", &proposal_id)
            .set("id", &proposal_id)
            .set("planName", plan_name)
            .set("planHeight", plan_height)
            .set("planInfo", plan_info)
            .set("proposal", proposal_id);
    }
}

pub fn insert_software_upgrade_proposal(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1Beta1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(software_upgrade_proposal) = <SoftwareUpgradeProposal as prost::Message>::decode(content.value.as_slice())
    {
        let proposer = msg.proposer.as_str();
        let (initial_deposit_denom, initial_deposit_amount) = extract_initial_deposit(&msg.initial_deposit);
        let authority = extract_authority(tx_result);

        let title = software_upgrade_proposal.title.as_str();
        let description = software_upgrade_proposal.description.as_str();

        let plan = software_upgrade_proposal.plan.unwrap();
        let plan_name = plan.name.as_str();
        let plan_height = plan.height;
        let plan_info = plan.info.as_str();

        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        // Create Proposal entity
        tables
            .create_row("Proposal", &proposal_id)
            .set("id", &proposal_id)
            .set("txHash", tx_hash)
            .set("blockNumber", clock.number)
            .set("type", "SoftwareUpgrade")
            .set("proposer", proposer)
            .set("initialDepositDenom", initial_deposit_denom)
            .set("initialDepositAmount", initial_deposit_amount)
            .set("authority", authority)
            .set("title", title)
            .set("description", description)
            .set("metadata", "");

        // Create SoftwareUpgradeProposal entity
        tables
            .create_row("SoftwareUpgradeProposal", &proposal_id)
            .set("id", &proposal_id)
            .set("planName", plan_name)
            .set("planHeight", plan_height)
            .set("planInfo", plan_info)
            .set("proposal", proposal_id);
    }
}
