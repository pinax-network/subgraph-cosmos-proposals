use crate::deposits::create_deposit;
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

                if let Some(content) = msg.content.as_ref() {
                    match content.type_url.as_str() {
                        "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade" => {
                            create_software_upgrade(tables, content, &proposal_id);
                        }
                        "/cosmos.distribution.v1beta1.MsgCommunityPoolSpend" => {
                            create_community_pool_spend(tables, content, &proposal_id);
                        }
                        _ => {}
                    }
                }
            }
        }
        "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
            if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(buf) {
                let row = tables.create_row("Proposal", &proposal_id);
                set_proposal_entity(row, clock, message, tx_result, tx_hash);
                set_proposal_v1beta1(row, &msg);

                if let Some(content) = msg.content.as_ref() {
                    match content.type_url.as_str() {
                        "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" => {
                            create_software_upgrade(tables, content, &proposal_id);
                        }
                        "/cosmos.params.v1beta1.ParameterChangeProposal" => {
                            create_parameter_change_proposal(tables, content, &proposal_id);
                        }
                        "/cosmos.distribution.v1beta1.CommunityPoolSpendProposal" => {
                            create_community_pool_spend(tables, content, &proposal_id);
                        }
                        "/ibc.core.client.v1.ClientUpdateProposal" => {
                            create_client_update(tables, content, &proposal_id);
                        }
                        _ => {}
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
    if let Some(content) = msg.content.as_ref() {
        let proposer = msg.proposer.as_str();
        let (title, summary) = decode_text_proposal(content);
        set_proposal_metadata(row, proposer, &title, &summary, "");
        set_content(row, &content);
    }
}

fn set_proposal_v1(row: &mut Row, msg: &MsgSubmitProposalV1) {
    let proposer = msg.proposer.as_str();
    let title = msg.title.as_str();
    let summary = msg.summary.as_str();
    let metadata = msg.metadata.as_str();
    set_proposal_metadata(row, proposer, title, summary, metadata);
    if let Some(content) = msg.content.as_ref() {
        set_content(row, &content);
    }
}

fn set_content(row: &mut Row, content: &Any) {
    row.set("content", Hex::encode(&content.value))
        .set("content_type", content.type_url.as_str());
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
