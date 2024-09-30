use crate::blocks::insert_block;
use crate::pb::cosmos::distribution::v1beta1::{CommunityPoolSpendProposal, MsgCommunityPoolSpend};
use crate::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use crate::proposal_deposits::insert_deposit;
use crate::utils::{extract_authority, extract_initial_deposit, extract_proposal_id};
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

pub fn insert_msg_community_pool_spend(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(msg_community_pool_spend) = <MsgCommunityPoolSpend as prost::Message>::decode(content.value.as_slice()) {
        let proposer = msg.proposer.as_str();
        let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

        let title = msg.title.as_str();
        let summary = msg.summary.as_str();
        let metadata = msg.metadata.as_str();

        let authority = msg_community_pool_spend.authority.as_str();
        let recipient = msg_community_pool_spend.recipient.as_str();
        let amount = msg_community_pool_spend.amount.get(0).unwrap();
        let amount_denom = amount.denom.as_str();
        let amount_amount = amount.amount.as_str();

        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        let data = serde_json::to_string(&serde_json::json!({
            "recipient": recipient,
            "amount": {
                "denom": amount_denom,
                "amount": amount_amount
            }
        }))
        .unwrap_or_default();

        insert_block(tables, clock);

        tables
            .create_row("Proposal", &proposal_id)
            .set("txHash", tx_hash)
            .set("block", &clock.id)
            .set("type", "CommunityPoolSpend")
            .set("proposer", proposer)
            .set("authority", authority)
            .set("title", title)
            .set("description", summary)
            .set("metadata", metadata);

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
            .set("typeUrl", "/cosmos.distribution.v1beta1.MsgCommunityPoolSpend")
            .set("jsonData", data)
            .set("proposal", &proposal_id);
    }
}

pub fn insert_community_pool_spend_proposal(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1Beta1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(comm_pool_spend_prop) = <CommunityPoolSpendProposal as prost::Message>::decode(content.value.as_slice()) {
        let proposer = msg.proposer.as_str();
        let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

        let title = comm_pool_spend_prop.title.as_str();
        let description = comm_pool_spend_prop.description.as_str();
        let recipient = comm_pool_spend_prop.recipient.as_str();
        let (amount_denom, amount_amount) = extract_initial_deposit(&comm_pool_spend_prop.amount);

        let authority = extract_authority(tx_result);

        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        let data = serde_json::to_string(&serde_json::json!({
            "recipient": recipient,
            "amount": {
                "denom": amount_denom,
                "amount": amount_amount
            }
        }))
        .unwrap_or_default();

        insert_block(tables, clock);
        tables
            .create_row("Proposal", &proposal_id)
            .set("txHash", tx_hash)
            .set("block", &clock.id)
            .set("type", "CommunityPoolSpend")
            .set("title", title)
            .set("description", description)
            .set("proposer", proposer)
            .set("authority", authority);

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
            .set("typeUrl", "/cosmos.gov.v1beta1.CommunityPoolSpendProposal")
            .set("jsonData", data.as_str());
    }
}
