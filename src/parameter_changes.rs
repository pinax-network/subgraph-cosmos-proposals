use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal;
use crate::pb::cosmos::params::v1beta1::{ParamChange, ParameterChangeProposal};
use crate::utils::{extract_authority, extract_initial_deposit, extract_proposal_id};
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

        let (initial_deposit_denom, initial_deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

        let title = parameter_change_proposal.title.as_str();
        let description = parameter_change_proposal.description.as_str();

        let authority = extract_authority(tx_result);

        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        let data = serde_json::to_string(&serde_json::json!({
            "changes": parameter_change_proposal.changes
        }))
        .unwrap_or_default();

        tables
            .create_row("Proposal", &proposal_id)
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
            .create_row("Content", &proposal_id)
            .set("proposal", &proposal_id)
            .set("typeUrl", "/cosmos.params.v1beta1.ParameterChangeProposal")
            .set("jsonData", data);
    }
}
