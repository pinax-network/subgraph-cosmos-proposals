use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::pb::{
    cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1, ibc::core::client::v1::ClientUpdateProposal,
};

pub fn insert_client_update_proposal(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(client_update_proposal) = <ClientUpdateProposal as prost::Message>::decode(content.value.as_slice()) {
        let proposer = msg_submit_proposal.proposer.as_str();
        let title = client_update_proposal.title.as_str();
        let description = client_update_proposal.description.as_str();
        let subject_client_id = client_update_proposal.subject_client_id.as_str();
        let substitute_client_id = client_update_proposal.substitute_client_id.as_str();

        let initial_deposit = msg_submit_proposal.initial_deposit.get(0).unwrap();
        let initial_deposit_denom = initial_deposit.denom.as_str();
        let initial_deposit_amount = initial_deposit.amount.as_str();
        let authority = tx_result
            .events
            .iter()
            .find(|event| event.r#type == "coin_received")
            .and_then(|event| event.attributes.iter().find(|attr| attr.key == "receiver"))
            .map(|attr| attr.value.as_str())
            .expect(&format!(
                "Authority not found for client update proposal at block {}",
                clock.number
            ));

        let data = serde_json::json!({
            "subject_client_id": subject_client_id,
            "substitute_client_id": substitute_client_id,
        });

        let proposal_id = tx_result
            .events
            .iter()
            .filter(|event| event.r#type == "submit_proposal")
            .flat_map(|event| event.attributes.iter())
            .find(|attr| attr.key == "proposal_id")
            .and_then(|attr| attr.value.parse::<u64>().ok())
            .expect(&format!(
                "proposal_id not found for client update proposal at block {} tx_hash {}",
                clock.number, tx_hash
            ));

        tables
            .create_row("Proposal", &proposal_id.to_string())
            .set("id", &proposal_id.to_string())
            .set("txHash", tx_hash)
            .set("blockNumber", clock.number)
            .set("type", "ClientUpdateProposal")
            .set("title", title)
            .set("description", description)
            .set("proposer", proposer)
            .set("authority", authority)
            .set("initialDepositDenom", initial_deposit_denom)
            .set("initialDepositAmount", initial_deposit_amount);

        tables
            .create_row("Content", &proposal_id.to_string())
            .set("id", &proposal_id.to_string())
            .set("proposal", &proposal_id.to_string())
            .set("typeUrl", content.type_url.as_str())
            .set("jsonData", data.to_string());
    }
}
