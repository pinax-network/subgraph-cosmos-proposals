use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::{
    pb::{
        cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1,
        ibc::core::client::v1::ClientUpdateProposal,
    },
    proposal_deposits::insert_deposit,
    utils::{
        extract_authority, extract_initial_deposit, extract_proposal_id, insert_content_entity_json,
        insert_proposal_entity,
    },
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
        let type_url = content.type_url.as_str();
        let proposer = msg.proposer.as_str();
        let title = client_update_proposal.title.as_str();
        let description = client_update_proposal.description.as_str();
        let subject_client_id = client_update_proposal.subject_client_id.as_str();
        let substitute_client_id = client_update_proposal.substitute_client_id.as_str();

        let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

        let authority = extract_authority(tx_result);

        let data = serde_json::json!({
            "subject_client_id": subject_client_id,
            "substitute_client_id": substitute_client_id,
        });

        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        insert_proposal_entity(
            tables,
            &proposal_id,
            tx_hash,
            clock,
            "ClientUpdate",
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

        insert_content_entity_json(tables, &proposal_id, type_url, data.to_string().as_str());
    }
}
