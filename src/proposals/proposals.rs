use crate::deposits::create_deposit;
use crate::pb::cosmos::authz::v1beta1::MsgExec;
use crate::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use crate::pb::cosmos::gov::v1beta1::{MsgSubmitProposal as MsgSubmitProposalV1Beta1, TextProposal};
use crate::utils::{extract_authority, extract_proposal_id};
use crate::votes::create_vote;
use prost::Message;
use prost_types::Any;

use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::{Row, Tables};

use super::client_update::create_client_update;
use super::community_pool_spends::create_community_pool_spend;
use super::parameter_changes::create_parameter_change_proposal;
use super::software_upgrades::create_software_upgrade;

pub fn handle_proposals(
    tables: &mut Tables,
    clock: &Clock,
    message: &prost_types::Any,
    tx_result: &TxResults,
    tx_hash: &str,
) {
    let buf = message.value.as_slice();
    let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

    match message.type_url.as_str() {
        "/cosmos.gov.v1.MsgSubmitProposal" => {
            if let Ok(msg) = MsgSubmitProposalV1::decode(buf) {
                let row = tables.create_row("Proposal", &proposal_id);
                set_proposal_entity(row, clock, message, tx_result, tx_hash);
                set_proposal_v1(row, &msg);
                set_proposal_messages(tables, &msg, &proposal_id);
            }
        }
        "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
            if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(buf) {
                let row = tables.create_row("Proposal", &proposal_id);
                set_proposal_entity(row, clock, message, tx_result, tx_hash);
                set_proposal_v1beta1(row, &msg);
                set_proposal_messages(tables, &msg, &proposal_id);

                if let Some(first_message) = msg.messages.first() {
                    match first_message.type_url.as_str() {
                        "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" => {
                            create_software_upgrade(tables, first_message, &proposal_id);
                        }
                        "/cosmos.params.v1beta1.ParameterChangeProposal" => {
                            create_parameter_change_proposal(tables, first_message, &proposal_id);
                        }
                        "/cosmos.distribution.v1beta1.CommunityPoolSpendProposal" => {
                            create_community_pool_spend(tables, first_message, &proposal_id);
                        }
                        "/ibc.core.client.v1.ClientUpdateProposal" => {
                            create_client_update(tables, first_message, &proposal_id);
                        }
                        _ => {}
                    }
                }
            }
        }
        "/cosmos.authz.v1beta1.MsgExec" => {
            if let Ok(msg_exec) = MsgExec::decode(buf) {
                for msg in msg_exec.msgs {
                    let row = tables.create_row("Proposal", &proposal_id);
                    if let Ok(msg) = MsgSubmitProposalV1::decode(msg.value.as_slice()) {
                        set_proposal_entity(row, clock, message, tx_result, tx_hash);
                        set_proposal_v1(row, &msg);
                        set_proposal_messages(tables, &msg, &proposal_id);
                    } else if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(msg.value.as_slice()) {
                        set_proposal_entity(row, clock, message, tx_result, tx_hash);
                        set_proposal_v1beta1(row, &msg);
                        set_proposal_messages(tables, &msg, &proposal_id);
                    }
                }
            }
        }
        "/cosmos.gov.v1beta1.MsgVote" => {
            create_vote(tables, message, &tx_result, &clock, &tx_hash);
        }
        "/cosmos.gov.v1beta1.MsgDeposit" => {
            create_deposit(tables, message, &clock, &tx_hash);
        }
        _ => {}
    }
}

fn set_proposal_v1beta1(row: &mut Row, msg: &MsgSubmitProposalV1Beta1) {
    if let Some(first_message) = msg.messages.first() {
        let proposer = msg.proposer.as_str();
        let (title, summary) = decode_text_proposal(first_message);
        set_proposal_metadata(row, proposer, &title, &summary, "", "Standard");
    }
}

fn set_proposal_v1(row: &mut Row, msg: &MsgSubmitProposalV1) {
    let proposer = msg.proposer.as_str();
    let title = msg.title.as_str();
    let summary = msg.summary.as_str();
    let metadata = msg.metadata.as_str();
    let proposal_type = msg.proposal_type.unwrap_or(-1);
    let mut type_str = "Standard".to_string();

    if proposal_type == -1 {
        // expedited is deprecated, but fallback to check if the proposal is expedited
        let expedited = msg.expedited.unwrap_or(false);
        if expedited {
            type_str = "Expedited".to_string();
        }
    } else {
        type_str = proposal_type_to_string(proposal_type);
    }

    set_proposal_metadata(row, proposer, title, summary, metadata, &type_str);
}

fn set_proposal_messages<T>(tables: &mut Tables, msg: &T, proposal_id: &str)
where
    T: HasMessages,
{
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
    fn get_messages(&self) -> &Vec<prost_types::Any>;
}

impl HasMessages for MsgSubmitProposalV1 {
    fn get_messages(&self) -> &Vec<prost_types::Any> {
        &self.messages
    }
}

impl HasMessages for MsgSubmitProposalV1Beta1 {
    fn get_messages(&self) -> &Vec<prost_types::Any> {
        &self.messages
    }
}

pub fn set_proposal_metadata(
    row: &mut Row,
    proposer: &str,
    title: &str,
    summary: &str,
    metadata: &str,
    proposal_type: &str,
) {
    row.set("proposer", proposer)
        .set("title", title)
        .set("summary", summary)
        .set("metadata", metadata)
        .set("proposal_type", proposal_type);
}

pub fn set_proposal_entity(
    row: &mut Row,
    clock: &Clock,
    message: &prost_types::Any,
    tx_result: &TxResults,
    tx_hash: &str,
) {
    let authority = extract_authority(tx_result);
    if message.type_url.to_string().len() == 0 {
        panic!("Empty type_url in proposal");
    }

    row.set("transaction", tx_hash)
        .set("block", &clock.id)
        .set("authority", authority)
        .set("type", message.type_url.to_string())
        .set("status", "DepositPeriod");
}

pub fn decode_text_proposal(content: &Any) -> (String, String) {
    let mut title = "".to_string();
    let mut description = "".to_string();
    if let Ok(partially_decoded) = <TextProposal as prost::Message>::decode(content.value.as_slice()) {
        title = partially_decoded.title;
        description = partially_decoded.description;
    }
    (title, description)
}

fn proposal_type_to_string(proposal_type: i32) -> String {
    match proposal_type {
        // 0 : PROPOSAL_TYPE_UNSPECIFIED defines no proposal type, which fallback to PROPOSAL_TYPE_STANDARD.
        0 => "Standard".to_string(),
        1 => "Standard".to_string(),
        2 => "MultipleChoice".to_string(),
        3 => "Optimistic".to_string(),
        4 => "Expedited".to_string(),
        _ => "Unknown".to_string(),
    }
}
