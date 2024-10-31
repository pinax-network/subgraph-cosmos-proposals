use prost::Message;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_cosmos::{pb::TxResults, Block};

use cosmos_proposals::pb::cosmos::gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1;
use cosmos_proposals::pb::cosmos::params::v1beta1::ParameterChangeProposal;
use cosmos_proposals::pb::cosmos::tx::v1beta1::Tx;
use cosmos_proposals::utils::{extract_proposal_id_from_tx, get_attribute_value};

use crate::pb::cosmos::custom_events::{GovParamsOptional, ProposalEvents};

#[substreams::handlers::map]
pub fn map_events(block: Block) -> Result<ProposalEvents, Error> {
    let mut proposal_events = ProposalEvents {
        gov_params_changes: extract_param_change_proposals(&block),
        passed_proposal_ids: extract_passed_proposal_ids(&block),
    };

    // Allows for gov_params to push the genesis parameters
    if block.height == 1 {
        proposal_events.passed_proposal_ids.push("-1".to_string());
    }

    Ok(proposal_events)
}

fn extract_passed_proposal_ids(block: &Block) -> Vec<String> {
    let mut proposals_passed = Vec::new();

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

fn extract_param_change_proposals(block: &Block) -> Vec<GovParamsOptional> {
    let mut param_change_proposals: Vec<GovParamsOptional> = Vec::new();

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

fn process_proposal_changes(proposal: &ParameterChangeProposal, proposal_id: String) -> Option<GovParamsOptional> {
    let gov_changes = proposal
        .changes
        .iter()
        .filter(|change| change.subspace == "gov")
        .collect::<Vec<_>>();

    if gov_changes.is_empty() {
        return None;
    }

    let mut gov_params = GovParamsOptional {
        proposal_id: proposal_id,
        deposit_params: None,
        voting_params: None,
        tally_params: None,
    };

    for gov_change in gov_changes {
        match gov_change.key.as_str() {
            "depositparams" => {
                gov_params.deposit_params = serde_json::from_str(gov_change.value.as_str()).ok();
            }
            "votingparams" => {
                gov_params.voting_params = serde_json::from_str(gov_change.value.as_str()).ok();
            }
            "tallyparams" => {
                gov_params.tally_params = serde_json::from_str(gov_change.value.as_str()).ok();
            }
            _ => {}
        }
    }

    Some(gov_params)
}
