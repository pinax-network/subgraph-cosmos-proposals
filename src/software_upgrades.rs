use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use crate::pb::cosmos::upgrade::v1beta1::{MsgSoftwareUpgrade, SoftwareUpgradeProposal};
use crate::proposal_deposits::insert_deposit;
use crate::proposals::insert_proposal_entity;
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
        let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

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
        insert_proposal_entity(
            tables,
            &proposal_id,
            tx_hash,
            clock,
            "SoftwareUpgrade",
            proposer,
            authority,
            title,
            summary,
            metadata,
        );

        insert_deposit(
            tables,
            &proposal_id,
            &deposit_amount,
            &deposit_denom,
            proposer,
            clock,
            tx_hash,
        );

        // Create SoftwareUpgradeProposal entity
        tables
            .create_row("SoftwareUpgradeProposal", &proposal_id)
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
        let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);
        let authority = extract_authority(tx_result);

        let title = software_upgrade_proposal.title.as_str();
        let description = software_upgrade_proposal.description.as_str();

        let plan = software_upgrade_proposal.plan.unwrap();
        let plan_name = plan.name.as_str();
        let plan_height = plan.height;
        let plan_info = plan.info.as_str();

        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        // Create Proposal entity
        insert_proposal_entity(
            tables,
            &proposal_id,
            tx_hash,
            clock,
            "SoftwareUpgrade",
            proposer,
            authority,
            title,
            description,
            "",
        );

        insert_deposit(
            tables,
            &proposal_id,
            &deposit_amount,
            &deposit_denom,
            proposer,
            clock,
            tx_hash,
        );

        // Create SoftwareUpgradeProposal entity
        tables
            .create_row("SoftwareUpgradeProposal", &proposal_id)
            .set("planName", plan_name)
            .set("planHeight", plan_height)
            .set("planInfo", plan_info)
            .set("proposal", proposal_id);
    }
}
