use serde::Deserialize;
use serde::Serialize;
use substreams::pb::substreams::Clock;
use substreams::store::Appender;
use substreams::store::StoreAppend;
use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetString;
use substreams_cosmos::pb::TxResults;
use substreams_cosmos::Block;

use crate::pb::cosmos::params::v1beta1::ParameterChangeProposal;
use crate::pb::cosmos::tx::v1beta1::Tx;
use crate::utils::extract_proposal_id_from_tx;
use crate::utils::get_attribute_value;

#[substreams::handlers::store]
pub fn store_gov_params(genesis_params: String, clock: Clock, block: Block, store: StoreAppend<String>) {
    if clock.number == 1 {
        store.append(0, "genesis_params", genesis_params);
        return;
    }

    let param_change_proposals = extract_param_change_proposals(&block);
    let proposals_passed = extract_proposals_passed(&block);

    // Store the proposals that change the governance parameters
    for proposal in param_change_proposals {
        store.append(0, "gov_param_proposals", serde_json::to_string(&proposal).unwrap());
    }

    // Store the IDs of the proposals that passed
    for proposal_id in proposals_passed {
        store.append(0, "passed_proposal_ids", proposal_id);
    }
}

fn extract_param_change_proposals(block: &Block) -> Vec<GovParams> {
    let mut param_change_proposals: Vec<GovParams> = Vec::new();

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
    let param_change = body
        .messages
        .iter()
        .find(|message| message.type_url == "/cosmos.params.v1beta1.ParameterChangeProposal")?;

    <ParameterChangeProposal as prost::Message>::decode(param_change.value.as_slice()).ok()
}

fn extract_proposals_passed(block: &Block) -> Vec<String> {
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

fn process_proposal_changes(proposal: &ParameterChangeProposal, proposal_id: String) -> Option<GovParams> {
    let gov_changes = proposal
        .changes
        .iter()
        .filter(|change| change.subspace == "gov")
        .collect::<Vec<_>>();

    if gov_changes.is_empty() {
        return None;
    }

    let mut gov_params = GovParams {
        proposal_id,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositParams {
    pub min_deposit: Option<Vec<Deposit>>,
    pub max_deposit_period: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deposit {
    pub denom: Option<String>,
    pub amount: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VotingParams {
    pub voting_period: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TallyParams {
    pub quorum: Option<String>,
    pub threshold: Option<String>,
    pub veto_threshold: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GovParams {
    pub proposal_id: String,
    pub deposit_params: Option<DepositParams>,
    pub voting_params: Option<VotingParams>,
    pub tally_params: Option<TallyParams>,
}
