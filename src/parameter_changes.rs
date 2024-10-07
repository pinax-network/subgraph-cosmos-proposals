use crate::blocks::insert_block;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal;
use crate::pb::cosmos::params::v1beta1::ParameterChangeProposal;
use crate::proposal_deposits::insert_deposit;
use crate::utils::{
    extract_authority, extract_initial_deposit, extract_proposal_id, insert_content_entity_json, insert_proposal_entity,
};
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

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
        let type_url = content.type_url.as_str();
        let proposer = msg.proposer.as_str();
        let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);
        let title = parameter_change_proposal.title.as_str();
        let description = parameter_change_proposal.description.as_str();
        let authority = extract_authority(tx_result);
        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);
        let data = serde_json::to_string(&serde_json::json!({"changes": parameter_change_proposal.changes}))
            .unwrap_or_default();

        insert_block(tables, clock);

        insert_proposal_entity(
            tables,
            &proposal_id,
            tx_hash,
            &clock.id,
            "ParameterChange",
            proposer,
            authority,
            title,
            description,
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

        insert_content_entity_json(tables, &proposal_id, type_url, &data);
    }
}
