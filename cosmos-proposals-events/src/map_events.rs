use cosmos_proposals_sinks::pb::cosmos::authz::v1beta1::MsgExec;
use cosmos_proposals_sinks::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use prost::Message;
use prost_types::Any;
use substreams::errors::Error;
use substreams_cosmos::{pb::TxResults, Block};

use cosmos_proposals_sinks::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use cosmos_proposals_sinks::pb::cosmos::params::v1beta1::ParameterChangeProposal;
use cosmos_proposals_sinks::pb::cosmos::tx::v1beta1::Tx;
use cosmos_proposals_sinks::utils::{extract_proposal_id_from_tx, get_attribute_value};

use cosmos_proposals_protobuf::pb::cosmos::proposals::v1::{
    Events, GovParamsChanges, GovParamsOptional, NewProposalWithType,
};

#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<Events, Error> {
    let mut events = Events {
        gov_params_changes: extract_param_change_proposals(&block),
        passed_proposal_ids: extract_passed_proposal_ids(&block),
        new_proposals_with_types: extract_new_proposals_with_types(&block),
    };

    // Allows for gov_params to push the genesis parameters
    if block.height == 1 {
        events.passed_proposal_ids.push("-1".to_string());
    }

    Ok(events)
}

fn extract_passed_proposal_ids(block: &Block) -> Vec<String> {
    let mut proposals_passed: Vec<String> = Vec::new();

    let proposal_updates = block.events.iter().filter(|event| event.r#type == "active_proposal");

    for proposal_update in proposal_updates {
        if let (Some(proposal_id), Some(status)) = (
            get_attribute_value(proposal_update, "proposal_id"),
            get_attribute_value(proposal_update, "proposal_result"),
        ) {
            if status == "proposal_passed" {
                proposals_passed.push(proposal_id);
            }
        }
    }

    proposals_passed
}

fn extract_param_change_proposals(block: &Block) -> Vec<GovParamsChanges> {
    let mut param_change_proposals: Vec<GovParamsChanges> = Vec::new();

    for (i, tx_result) in block.tx_results.iter().enumerate() {
        if let Some(proposal) = extract_parameter_change_proposal(tx_result, &block.txs[i]) {
            let proposal_id = extract_proposal_id_from_tx(tx_result).unwrap();
            if let Some(gov_params) = process_proposal_changes(&proposal, proposal_id) {
                param_change_proposals.push(gov_params);
            }
        }
    }
    param_change_proposals
}

fn extract_parameter_change_proposal(tx_result: &TxResults, tx_bytes: &[u8]) -> Option<ParameterChangeProposal> {
    // Skip if not a proposal submission
    if !tx_result.events.iter().any(|event| event.r#type == "submit_proposal") {
        return None;
    }

    // Try to decode transaction
    let tx = match <Tx as prost::Message>::decode(tx_bytes) {
        Ok(tx) => tx,
        Err(_) => return None,
    };

    // Look for parameter change proposal in messages
    let body = tx.body?;

    for message in body.messages.iter() {
        let param_change = match message.type_url.as_str() {
            "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
                let msg = MsgSubmitProposalV1Beta1::decode(message.value.as_slice()).ok()?;
                msg.messages
                    .into_iter()
                    .find(|msg| msg.type_url == "/cosmos.params.v1beta1.ParameterChangeProposal")
            }
            _ => continue,
        };

        if let Some(param_change) = param_change {
            if let Ok(proposal) = <ParameterChangeProposal as prost::Message>::decode(param_change.value.as_slice()) {
                return Some(proposal);
            }
        }
    }
    None
}

fn extract_new_proposals_with_types(block: &Block) -> Vec<NewProposalWithType> {
    let mut new_proposal_types: Vec<NewProposalWithType> = Vec::new();

    for (i, tx_result) in block.tx_results.iter().enumerate() {
        let tx = match decode_transaction(&block.txs[i]) {
            Some(tx) => tx,
            None => continue,
        };

        let body = tx.body.expect("Tx body is required");
        for message in body.messages.iter() {
            if let Some(proposal_type) = get_proposal_type(message, tx_result) {
                new_proposal_types.push(proposal_type);
            }
        }
    }
    new_proposal_types
}

fn decode_transaction(tx_bytes: &[u8]) -> Option<Tx> {
    <Tx as prost::Message>::decode(tx_bytes).ok()
}

fn get_proposal_type(message: &Any, tx_result: &TxResults) -> Option<NewProposalWithType> {
    let proposal_id = extract_proposal_id_from_tx(tx_result).unwrap_or_default();

    match message.type_url.as_str() {
        "/cosmos.gov.v1beta1.MsgSubmitProposal" => Some(NewProposalWithType {
            proposal_id,
            proposal_type: "Standard".to_string(),
        }),
        "/cosmos.gov.v1.MsgSubmitProposal" => {
            let msg = MsgSubmitProposalV1::decode(message.value.as_slice()).expect("Invalid proposal message");
            Some(NewProposalWithType {
                proposal_id,
                proposal_type: determine_proposal_type(&msg),
            })
        }
        "/cosmos.authz.v1beta1.MsgExec" => {
            if let Ok(msg_exec) = MsgExec::decode(message.value.as_slice()) {
                for msg in msg_exec.msgs {
                    if let Ok(msg) = MsgSubmitProposalV1::decode(msg.value.as_slice()) {
                        return Some(NewProposalWithType {
                            proposal_id,
                            proposal_type: determine_proposal_type(&msg),
                        });
                    } else if let Ok(_) = MsgSubmitProposalV1Beta1::decode(msg.value.as_slice()) {
                        return Some(NewProposalWithType {
                            proposal_id,
                            proposal_type: "Standard".to_string(),
                        });
                    }
                }
            }
            None
        }
        _ => None,
    }
}

fn process_proposal_changes(proposal: &ParameterChangeProposal, proposal_id: String) -> Option<GovParamsChanges> {
    let gov_changes = proposal
        .changes
        .iter()
        .filter(|change| change.subspace == "gov")
        .collect::<Vec<_>>();

    if gov_changes.is_empty() {
        return None;
    }

    let mut param_changes = GovParamsChanges {
        proposal_id: proposal_id,
        params: Some(GovParamsOptional {
            deposit_params: None,
            voting_params: None,
            tally_params: None,
        }),
    };

    for gov_change in gov_changes {
        if let Some(params) = &mut param_changes.params {
            match gov_change.key.as_str() {
                "depositparams" => {
                    params.deposit_params = serde_json::from_str(gov_change.value.as_str()).ok();
                }
                "votingparams" => {
                    params.voting_params = serde_json::from_str(gov_change.value.as_str()).ok();
                }
                "tallyparams" => {
                    params.tally_params = serde_json::from_str(gov_change.value.as_str()).ok();
                }
                _ => {}
            }
        }
    }

    Some(param_changes)
}

fn determine_proposal_type(msg: &MsgSubmitProposalV1) -> String {
    match msg.proposal_type.unwrap_or(-1) {
        -1 => {
            if msg.expedited.unwrap_or(false) {
                "Expedited".to_string()
            } else {
                "Standard".to_string()
            }
        }
        proposal_type => proposal_type_to_string(proposal_type),
    }
}

fn proposal_type_to_string(proposal_type: i32) -> String {
    match proposal_type {
        0 | 1 => "Standard",
        2 => "MultipleChoice",
        3 => "Optimistic",
        4 => "Expedited",
        _ => "Unknown",
    }
    .to_string()
}
