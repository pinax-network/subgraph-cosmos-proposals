use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::{
    pb::cosmos::gov::{
        v1::MsgSubmitProposal as MsgSubmitProposalV1, v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1,
    },
    utils::{extract_authority, extract_initial_deposit, extract_proposal_id},
};

pub fn insert_other_proposal_v1(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    let type_url = content.type_url.as_str();
    let proposer = msg.proposer.as_str();
    let title = msg.title.as_str();
    let description = msg.summary.as_str();
    let metadata = msg.metadata.as_str();

    let (initial_deposit_denom, initial_deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

    let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

    let authority = extract_authority(tx_result);

    let raw_data = Hex::encode(content.value.as_slice());

    tables
        .create_row("Proposal", &proposal_id)
        .set("id", &proposal_id)
        .set("txHash", tx_hash)
        .set("blockNumber", clock.number)
        .set("type", "Undecoded Proposal")
        .set("proposer", proposer)
        .set("initialDepositDenom", initial_deposit_denom)
        .set("initialDepositAmount", initial_deposit_amount)
        .set("authority", authority)
        .set("title", title)
        .set("description", description)
        .set("metadata", metadata);

    tables
        .create_row("Content", &proposal_id)
        .set("id", &proposal_id)
        .set("proposal", &proposal_id)
        .set("typeUrl", type_url)
        .set("value", raw_data);
}

pub fn insert_other_proposal_v1beta1(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1Beta1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    let type_url = content.type_url.as_str();
    let proposer = msg.proposer.as_str();

    let (initial_deposit_denom, initial_deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

    let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

    let authority = extract_authority(tx_result);

    let mut title = "".to_string();
    let mut description = "".to_string();

    if let Ok(partially_decoded) = <PartiallyDecodedProposalV1Beta1 as prost::Message>::decode(content.value.as_slice())
    {
        title = partially_decoded.title;
        description = partially_decoded.description;
    }

    let raw_data = Hex::encode(content.value.as_slice());

    tables
        .create_row("Proposal", &proposal_id)
        .set("id", &proposal_id)
        .set("txHash", tx_hash)
        .set("blockNumber", clock.number)
        .set("type", "Undecoded Proposal")
        .set("proposer", proposer)
        .set("initialDepositDenom", initial_deposit_denom)
        .set("initialDepositAmount", initial_deposit_amount)
        .set("authority", authority)
        .set("title", title)
        .set("description", description);

    tables
        .create_row("Content", &proposal_id)
        .set("id", &proposal_id)
        .set("proposal", &proposal_id)
        .set("typeUrl", type_url)
        .set("value", raw_data);
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartiallyDecodedProposalV1Beta1 {
    /// title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
