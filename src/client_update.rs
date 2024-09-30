use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::{
    blocks::insert_block,
    pb::{
        cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1,
        ibc::core::client::v1::ClientUpdateProposal,
    },
    utils::{extract_authority, extract_initial_deposit, extract_proposal_id},
};

pub fn insert_client_update_proposal(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1Beta1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(client_update_proposal) = <ClientUpdateProposal as prost::Message>::decode(content.value.as_slice()) {
        let proposer = msg.proposer.as_str();
        let title = client_update_proposal.title.as_str();
        let description = client_update_proposal.description.as_str();
        let subject_client_id = client_update_proposal.subject_client_id.as_str();
        let substitute_client_id = client_update_proposal.substitute_client_id.as_str();

        let (initial_deposit_denom, initial_deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

        let authority = extract_authority(tx_result);

        let data = serde_json::json!({
            "subject_client_id": subject_client_id,
            "substitute_client_id": substitute_client_id,
        });

        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        insert_block(tables, clock);

        tables
            .create_row("Proposal", &proposal_id)
            .set("txHash", tx_hash)
            .set("block", &clock.id)
            .set("type", "ClientUpdate")
            .set("title", title)
            .set("description", description)
            .set("proposer", proposer)
            .set("authority", authority)
            .set("initialDepositDenom", initial_deposit_denom)
            .set("initialDepositAmount", initial_deposit_amount);

        tables
            .create_row("Content", &proposal_id)
            .set("proposal", &proposal_id)
            .set("typeUrl", &content.type_url)
            .set("jsonData", data.to_string());
    }
}
