use crate::pb::cosmos::distribution::v1beta1::MsgCommunityPoolSpend;
use crate::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

pub fn insert_msg_community_pool_spend(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(msg_community_pool_spend) = <MsgCommunityPoolSpend as prost::Message>::decode(content.value.as_slice()) {
        let proposer = msg_submit_proposal.proposer.as_str();
        let initial_deposit = msg_submit_proposal.initial_deposit.get(0).unwrap();
        let initial_deposit_denom = initial_deposit.denom.as_str();
        let initial_deposit_amount = initial_deposit.amount.as_str();

        let title = msg_submit_proposal.title.as_str();
        let summary = msg_submit_proposal.summary.as_str();
        let metadata = msg_submit_proposal.metadata.as_str();

        let authority = msg_community_pool_spend.authority.as_str();
        let recipient = msg_community_pool_spend.recipient.as_str();
        let amount = msg_community_pool_spend.amount.get(0).unwrap();
        let amount_denom = amount.denom.as_str();
        let amount_amount = amount.amount.as_str();

        let proposal_id = tx_result
            .events
            .iter()
            .filter(|event| event.r#type == "submit_proposal")
            .flat_map(|event| event.attributes.iter())
            .find(|attr| attr.key == "proposal_id")
            .and_then(|attr| attr.value.parse::<u64>().ok())
            .expect(&format!(
                "proposal_id not found for parameter change proposal at block {}",
                clock.number
            ));

        let data = serde_json::to_string(&serde_json::json!({
            "recipient": recipient,
            "amount": {
                "denom": amount_denom,
                "amount": amount_amount
            }
        }))
        .unwrap_or_default();

        tables
            .create_row("Proposal", &proposal_id.to_string())
            .set("id", &proposal_id.to_string())
            .set("txHash", tx_hash)
            .set("blockNumber", clock.number)
            .set("type", "CommunityPoolSpendProposal")
            .set("proposer", proposer)
            .set("authority", authority)
            .set("initialDepositDenom", initial_deposit_denom)
            .set("initialDepositAmount", initial_deposit_amount)
            .set("title", title)
            .set("description", summary)
            .set("metadata", metadata);

        tables
            .create_row("Content", &proposal_id.to_string())
            .set("id", &proposal_id.to_string())
            .set("proposal", &proposal_id.to_string())
            .set("typeUrl", "/cosmos.distribution.v1beta1.MsgCommunityPoolSpend")
            .set("jsonData", data.as_str());
    }
}

pub fn insert_community_pool_spend_proposal(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
}
