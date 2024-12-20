use crate::deposits::{create_deposit_msg, create_initial_deposit};
use crate::pb::cosmos::{
    authz::v1beta1::MsgExec,
    gov::v1::MsgSubmitProposal as MsgSubmitProposalV1,
    gov::v1beta1::{MsgSubmitProposal as MsgSubmitProposalV1Beta1, TextProposal},
};
use crate::utils::{
    add_nanoseconds_to_timestamp, determine_voting_end_time, extract_authority, extract_proposal_id,
    extract_proposal_status, get_proposal_type, GovernanceParamsStore,
};
use crate::votes::create_vote;
use prost::Message;
use prost_types::{Any, Timestamp};
use substreams::{pb::substreams::Clock, Hex};
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::{Row, Tables};

use super::{
    client_update::create_client_update, community_pool_spends::create_community_pool_spend,
    parameter_changes::create_parameter_change_proposal, software_upgrades::create_software_upgrade,
};

pub fn handle_proposals(
    tables: &mut Tables,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    gov_params: &GovernanceParamsStore,
) {
    let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);
    let status = extract_proposal_status(tx_result);

    match message.type_url.as_str() {
        "/cosmos.gov.v1.MsgSubmitProposal" => handle_v1_proposal(
            tables,
            clock,
            message,
            tx_result,
            tx_hash,
            &proposal_id,
            &status,
            gov_params,
        ),
        "/cosmos.gov.v1beta1.MsgSubmitProposal" => handle_v1beta1_proposal(
            tables,
            clock,
            message,
            tx_result,
            tx_hash,
            &proposal_id,
            &status,
            gov_params,
        ),
        "/cosmos.authz.v1beta1.MsgExec" => handle_exec_proposal(
            tables,
            clock,
            message,
            tx_result,
            tx_hash,
            &proposal_id,
            &status,
            gov_params,
        ),
        "/cosmos.gov.v1beta1.MsgVote" => create_vote(tables, message, tx_result, clock, tx_hash),
        "/cosmos.gov.v1beta1.MsgDeposit" => create_deposit_msg(tables, message, clock, tx_result, tx_hash, gov_params),
        _ => {}
    }
}

fn handle_v1_proposal(
    tables: &mut Tables,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    proposal_id: &str,
    status: &str,
    gov_params: &GovernanceParamsStore,
) {
    if let Ok(msg) = MsgSubmitProposalV1::decode(message.value.as_slice()) {
        let row = tables.create_row("Proposal", proposal_id);
        set_proposal_entity(row, clock, message, tx_result, tx_hash, proposal_id, status, gov_params);
        set_proposal_v1(row, &msg);
        set_proposal_messages(tables, &msg, proposal_id);
        create_initial_deposit(tables, clock, tx_result, tx_hash, proposal_id, &msg.proposer);
    }
}

fn handle_v1beta1_proposal(
    tables: &mut Tables,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    proposal_id: &str,
    status: &str,
    gov_params: &GovernanceParamsStore,
) {
    if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(message.value.as_slice()) {
        let row = tables.create_row("Proposal", proposal_id);
        set_proposal_entity(row, clock, message, tx_result, tx_hash, proposal_id, status, gov_params);
        set_proposal_v1beta1(row, &msg);
        set_proposal_messages(tables, &msg, proposal_id);
        create_initial_deposit(tables, clock, tx_result, tx_hash, proposal_id, &msg.proposer);

        if let Some(first_message) = msg.messages.first() {
            handle_specific_proposal(tables, first_message, proposal_id);
        }
    }
}

fn handle_exec_proposal(
    tables: &mut Tables,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    proposal_id: &str,
    status: &str,
    gov_params: &GovernanceParamsStore,
) {
    if let Ok(msg_exec) = MsgExec::decode(message.value.as_slice()) {
        for msg in msg_exec.msgs {
            let row = tables.create_row("Proposal", proposal_id);
            set_proposal_entity(row, clock, message, tx_result, tx_hash, proposal_id, status, gov_params);

            if let Ok(msg) = MsgSubmitProposalV1::decode(msg.value.as_slice()) {
                set_proposal_v1(row, &msg);
                set_proposal_messages(tables, &msg, proposal_id);
                create_initial_deposit(tables, clock, tx_result, tx_hash, proposal_id, &msg.proposer);
            } else if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(msg.value.as_slice()) {
                set_proposal_v1beta1(row, &msg);
                set_proposal_messages(tables, &msg, proposal_id);
                create_initial_deposit(tables, clock, tx_result, tx_hash, proposal_id, &msg.proposer);
            }
        }
    }
}

fn handle_specific_proposal(tables: &mut Tables, message: &Any, proposal_id: &str) {
    match message.type_url.as_str() {
        "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" => create_software_upgrade(tables, message, proposal_id),
        "/cosmos.params.v1beta1.ParameterChangeProposal" => {
            create_parameter_change_proposal(tables, message, proposal_id)
        }
        "/cosmos.distribution.v1beta1.CommunityPoolSpendProposal" => {
            create_community_pool_spend(tables, message, proposal_id)
        }
        "/ibc.core.client.v1.ClientUpdateProposal" => create_client_update(tables, message, proposal_id),
        _ => {}
    }
}

fn set_proposal_v1beta1(row: &mut Row, msg: &MsgSubmitProposalV1Beta1) {
    if let Some(first_message) = msg.messages.first() {
        let proposer = msg.proposer.as_str();
        let (title, summary) = decode_text_proposal(first_message);
        set_proposal_metadata(row, proposer, &title, &summary, "");
    }
}

fn set_proposal_v1(row: &mut Row, msg: &MsgSubmitProposalV1) {
    let proposer = msg.proposer.as_str();
    let title = msg.title.as_str();
    let summary = msg.summary.as_str();
    let metadata = msg.metadata.as_str();
    set_proposal_metadata(row, proposer, title, summary, metadata);
}

fn set_proposal_messages<T: HasMessages>(tables: &mut Tables, msg: &T, proposal_id: &str) {
    for (i, message) in msg.get_messages().iter().enumerate() {
        let id = format!("{}-{}", proposal_id, i);
        tables
            .create_row("ProposalMessage", &id)
            .set("message_index", i as u8)
            .set("type", message.type_url.as_str())
            .set("raw_data", Hex::encode(&message.value))
            .set("proposal", proposal_id);
    }
}

trait HasMessages {
    fn get_messages(&self) -> &Vec<Any>;
}

impl HasMessages for MsgSubmitProposalV1 {
    fn get_messages(&self) -> &Vec<Any> {
        &self.messages
    }
}

impl HasMessages for MsgSubmitProposalV1Beta1 {
    fn get_messages(&self) -> &Vec<Any> {
        &self.messages
    }
}

pub fn set_proposal_metadata(row: &mut Row, proposer: &str, title: &str, summary: &str, metadata: &str) {
    row.set("proposer", proposer)
        .set("title", title)
        .set("summary", summary)
        .set("metadata", metadata);
}

pub fn set_proposal_entity(
    row: &mut Row,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    proposal_id: &str,
    status: &str,
    gov_params: &GovernanceParamsStore,
) {
    let authority = extract_authority(tx_result);
    if message.type_url.is_empty() {
        panic!("Empty type_url in proposal");
    }

    let submit_time = clock.timestamp.as_ref().expect("missing submit_time");

    let proposal_type = &get_proposal_type(gov_params, proposal_id);

    row.set("transaction", tx_hash)
        .set("block", &clock.id)
        .set("authority", authority)
        .set("type", &message.type_url)
        .set("status", status)
        .set("submit_time", submit_time)
        .set("proposal_type", proposal_type);

    set_proposal_gov_params(row, gov_params, submit_time, status, proposal_type);
}

fn set_proposal_gov_params(
    row: &mut Row,
    gov_params: &GovernanceParamsStore,
    submit_time: &Timestamp,
    status: &str,
    proposal_type: &str,
) {
    let deposit_end_time = add_nanoseconds_to_timestamp(submit_time, &gov_params.max_deposit_period);

    row.set("deposit_end_time", &deposit_end_time)
        .set("governance_parameter", &gov_params.block_id_last_updated);

    if status == "VotingPeriod" {
        // Default to standard voting period if proposal type is not expedited
        let voting_end_time = determine_voting_end_time(gov_params, submit_time, proposal_type);

        row.set("voting_start_time", submit_time)
            .set("voting_end_time", &voting_end_time);
    }
}

pub fn decode_text_proposal(content: &Any) -> (String, String) {
    TextProposal::decode(content.value.as_slice())
        .map(|decoded| (decoded.title, decoded.description))
        .unwrap_or_default()
}
