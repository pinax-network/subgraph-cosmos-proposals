use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use crate::pb::cosmos::gov::v1beta1::TextProposal;
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

pub fn insert_text_proposal(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(text_prop) = <TextProposal as prost::Message>::decode(content.value.as_slice()) {
        let title = text_prop.title.as_str();
        let description = text_prop.description.as_str();
        let proposer = msg_submit_proposal.proposer.as_str();

        let proposal_id = tx_result
            .events
            .iter()
            .filter(|event| event.r#type == "submit_proposal")
            .flat_map(|event| event.attributes.iter())
            .find(|attr| attr.key == "proposal_id")
            .and_then(|attr| attr.value.parse::<u64>().ok())
            .expect(&format!(
                "Proposal_id not found for text proposal at block {}",
                clock.number
            ));

        let initial_deposit = msg_submit_proposal.initial_deposit.get(0).unwrap();
        let initial_deposit_denom = initial_deposit.denom.as_str();
        let initial_deposit_amount = initial_deposit.amount.as_str();

        let authority = tx_result
            .events
            .iter()
            .find(|event| event.r#type == "coin_received")
            .and_then(|event| event.attributes.iter().find(|attr| attr.key == "receiver"))
            .map(|attr| attr.value.as_str())
            .unwrap_or("");

        tables
            .create_row("Proposal", &proposal_id.to_string())
            .set("id", &proposal_id.to_string())
            .set("txHash", tx_hash)
            .set("proposer", proposer)
            .set("authority", authority)
            .set("blockNumber", clock.number)
            .set("type", "TextProposal")
            .set("initialDepositDenom", initial_deposit_denom)
            .set("initialDepositAmount", initial_deposit_amount)
            .set("title", title)
            .set("description", description);
    }
}
