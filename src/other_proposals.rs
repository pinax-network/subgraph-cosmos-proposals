use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::{
    pb::cosmos::gov::{
        v1::MsgSubmitProposal as MsgSubmitProposalV1, v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1,
    },
    proposal_deposits::insert_deposit,
    utils::{
        extract_authority, extract_initial_deposit, extract_proposal_id, insert_content_entity_raw_data,
        insert_proposal_entity,
    },
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

    let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

    let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

    let authority = extract_authority(tx_result);

    let raw_data = Hex::encode(content.value.as_slice());

    insert_proposal_entity(
        tables,
        &proposal_id,
        tx_hash,
        clock,
        "Undecoded Proposal",
        proposer,
        authority,
        title,
        description,
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

    tables
        .create_row("Content", &proposal_id)
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

    let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

    let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

    let authority = extract_authority(tx_result);

    let mut title = "".to_string();
    let mut description = "".to_string();

    if let Ok(partially_decoded) = <PartiallyDecodedProposalV1Beta1 as prost::Message>::decode(content.value.as_slice())
    {
        title = partially_decoded.title;
        description = partially_decoded.description;
    }

    let value = Hex::encode(content.value.as_slice());

    insert_proposal_entity(
        tables,
        &proposal_id,
        tx_hash,
        clock,
        "Undecoded Proposal",
        proposer,
        authority,
        title.as_str(),
        description.as_str(),
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

    insert_content_entity_raw_data(tables, &proposal_id, type_url, value.as_str());
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
