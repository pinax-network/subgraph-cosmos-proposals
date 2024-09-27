use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal;
use crate::pb::cosmos::params::v1beta1::{ParamChange, ParameterChangeProposal};
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

pub fn insert_parameter_change_proposal(
    tables: &mut Tables,
    msg: &MsgSubmitProposal,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(parameter_change_proposal) = <ParameterChangeProposal as prost::Message>::decode(content.value.as_slice())
    {
        let proposer = msg.proposer.as_str();
        let initial_deposit = msg.initial_deposit.get(0).unwrap();
        let initial_deposit_denom = initial_deposit.denom.as_str();
        let initial_deposit_amount = initial_deposit.amount.as_str();

        let title = parameter_change_proposal.title.as_str();
        let description = parameter_change_proposal.description.as_str();

        let authority = tx_result
            .events
            .iter()
            .find(|event| event.r#type == "coin_received")
            .and_then(|event| event.attributes.iter().find(|attr| attr.key == "receiver"))
            .map(|attr| attr.value.as_str())
            .unwrap_or("");

        let proposal_id = tx_result
            .events
            .iter()
            .filter(|event| event.r#type == "submit_proposal")
            .flat_map(|event| event.attributes.iter())
            .find(|attr| attr.key == "proposal_id")
            .and_then(|attr| attr.value.parse::<u64>().ok())
            // Start of Selection
            .expect(&format!(
                "proposal_id not found for parameter change proposal at block {}",
                clock.number
            ));

        let data = serde_json::to_string(&serde_json::json!({
            "changes": parameter_change_proposal.changes
        }))
        .unwrap_or_default();

        tables
            .create_row("Proposal", &proposal_id.to_string())
            .set("id", &proposal_id.to_string())
            .set("txHash", tx_hash)
            .set("blockNumber", clock.number)
            .set("type", "ParameterChange")
            .set("proposer", proposer)
            .set("authority", authority)
            .set("initialDepositDenom", initial_deposit_denom)
            .set("initialDepositAmount", initial_deposit_amount)
            .set("title", title)
            .set("description", description);

        tables
            .create_row("Content", &proposal_id.to_string())
            .set("id", &proposal_id.to_string())
            .set("proposal", &proposal_id.to_string())
            .set("typeUrl", "/cosmos.params.v1beta1.ParameterChangeProposal")
            .set("jsonData", data.as_str());
    }
}
