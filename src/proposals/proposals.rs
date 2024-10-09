use crate::deposits::insert_deposit_undecoded;
use crate::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use crate::votes::push_proposal_vote;
use prost::Message;

use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::order_by::insert_order_by;

use super::client_update::insert_client_update_proposal;
use super::community_pool_spends::{insert_community_pool_spend_proposal, insert_msg_community_pool_spend};
use super::other_proposals::{insert_other_proposal_v1, insert_other_proposal_v1beta1};
use super::parameter_changes::insert_parameter_change_proposal;
use super::software_upgrades::{insert_message_software_upgrade, insert_software_upgrade_proposal};
use super::text_proposal::insert_text_proposal;

pub fn handle_proposals(
    message: &prost_types::Any,
    tables: &mut Tables,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    match message.type_url.as_str() {
        "/cosmos.gov.v1.MsgSubmitProposal" => {
            if let Ok(msg) = MsgSubmitProposalV1::decode(message.value.as_slice()) {
                push_proposal_v1(tables, &msg, &tx_result, &clock, &tx_hash);
            }
        }
        "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
            if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(message.value.as_slice()) {
                push_proposal_v1beta1(tables, &msg, &tx_result, &clock, &tx_hash);
            }
        }
        "/cosmos.gov.v1beta1.MsgVote" => {
            push_proposal_vote(tables, message, &tx_result, &clock, &tx_hash);
        }
        "/cosmos.gov.v1beta1.MsgDeposit" => {
            insert_deposit_undecoded(tables, message, &clock, &tx_hash);
        }
        _ => {}
    }
}

fn push_proposal_v1beta1(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1Beta1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Some(content) = msg.content.as_ref() {
        match content.type_url.as_str() {
            "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" => {
                insert_software_upgrade_proposal(tables, msg, content, tx_result, clock, tx_hash);
            }
            "/cosmos.params.v1beta1.ParameterChangeProposal" => {
                insert_parameter_change_proposal(tables, msg, content, tx_result, clock, tx_hash);
            }
            "/cosmos.distribution.v1beta1.CommunityPoolSpendProposal" => {
                insert_community_pool_spend_proposal(tables, msg, content, tx_result, clock, tx_hash);
            }
            "/cosmos.gov.v1beta1.TextProposal" => {
                insert_text_proposal(tables, msg, content, tx_result, clock, tx_hash);
            }
            "/ibc.core.client.v1.ClientUpdateProposal" => {
                insert_client_update_proposal(tables, msg, content, tx_result, clock, tx_hash);
            }
            _ => {
                insert_other_proposal_v1beta1(tables, msg, content, tx_result, clock, tx_hash);
            }
        }
    }
}

fn push_proposal_v1(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Some(content) = msg.content.as_ref() {
        match content.type_url.as_str() {
            "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade" => {
                insert_message_software_upgrade(tables, msg, content, tx_result, clock, tx_hash);
            }
            "/cosmos.distribution.v1beta1.MsgCommunityPoolSpend" => {
                insert_msg_community_pool_spend(tables, msg, content, tx_result, clock, tx_hash);
            }
            _ => {
                insert_other_proposal_v1(tables, msg, content, tx_result, clock, tx_hash);
            }
        }
    }
}

pub fn insert_proposal_entity(
    tables: &mut Tables,
    id: &str,
    tx_hash: &str,
    clock: &Clock,
    proposal_type: &str,
    proposer: &str,
    authority: &str,
    title: &str,
    description: &str,
    metadata: &str,
) {
    let row = tables
        .create_row("Proposal", id)
        .set("txHash", tx_hash)
        .set("type", proposal_type)
        .set("status", "voting_period")
        .set("proposer", proposer)
        .set("authority", authority)
        .set("title", title)
        .set("description", description)
        .set("metadata", metadata);

    insert_order_by(row, clock);
}
