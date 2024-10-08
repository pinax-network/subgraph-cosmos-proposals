use crate::deposits::insert_deposit;
use crate::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use crate::pb::cosmos::gov::v1beta1::TextProposal;
use crate::proposals::insert_proposal_entity;
use crate::utils::extract_authority;
use crate::utils::extract_initial_deposit;
use crate::utils::extract_proposal_id;
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

pub fn insert_text_proposal(
    tables: &mut Tables,
    msg: &MsgSubmitProposalV1Beta1,
    content: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(text_prop) = <TextProposal as prost::Message>::decode(content.value.as_slice()) {
        let title = text_prop.title.as_str();
        let description = text_prop.description.as_str();
        let proposer = msg.proposer.as_str();

        let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);

        let (deposit_denom, deposit_amount) = extract_initial_deposit(&msg.initial_deposit);

        let authority = extract_authority(tx_result);

        insert_proposal_entity(
            tables,
            &proposal_id,
            tx_hash,
            clock,
            "Text",
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
    }
}
