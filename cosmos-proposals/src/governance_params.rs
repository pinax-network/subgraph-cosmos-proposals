use serde::{Deserialize, Serialize};
use substreams_entity_change::tables::{Row, Tables};

use crate::utils::GovernanceParamsStore;

pub fn push_governance_params(tables: &mut Tables, gov_params: &GovernanceParamsStore) {
    let block_id = &gov_params.block_id_last_updated;

    tables
        .create_row("GovernanceParameter", &gov_params.block_id_last_updated)
        .set("id", &gov_params.block_id_last_updated)
        .set("block", block_id);

    create_deposit_params(tables, block_id, gov_params);
    create_voting_params(tables, block_id, gov_params);
    create_tally_params(tables, block_id, gov_params);
}

fn add_governance_parameter_derive_from(row: &mut Row, block_id: &str, id: &str) {
    row.set("block", block_id).set("governance_parameter", id);
}

fn create_deposit_params(tables: &mut Tables, block_number: &str, gov_params: &GovernanceParamsStore) {
    let mut row = tables
        .create_row("DepositParam", &gov_params.block_id_last_updated)
        .set("min_deposit", &gov_params.min_deposit)
        .set_bigint("max_deposit_period", &gov_params.max_deposit_period);

    if let Some(expedited_min_deposit) = &gov_params.expedited_min_deposit {
        row.set("expedited_min_deposit", expedited_min_deposit);
    }

    add_governance_parameter_derive_from(row, &block_number, &gov_params.block_id_last_updated);
}

fn create_voting_params(tables: &mut Tables, block_number: &str, gov_params: &GovernanceParamsStore) {
    let mut row = tables
        .create_row("VotingParam", &gov_params.block_id_last_updated)
        .set_bigint("voting_period", &gov_params.voting_period);

    if let Some(expedited_voting_period) = &gov_params.expedited_voting_period {
        row.set_bigint("expedited_voting_period", expedited_voting_period);
    }

    add_governance_parameter_derive_from(&mut row, block_number, &gov_params.block_id_last_updated);
}

fn create_tally_params(tables: &mut Tables, block_number: &str, gov_params: &GovernanceParamsStore) {
    let mut row = tables
        .create_row("TallyParam", &gov_params.block_id_last_updated)
        .set_bigdecimal("quorum", &gov_params.quorum)
        .set_bigdecimal("threshold", &gov_params.threshold)
        .set_bigdecimal("veto_threshold", &gov_params.veto_threshold);

    if let Some(expedited_quorum) = &gov_params.expedited_quorum {
        row.set_bigdecimal("expedited_quorum", expedited_quorum);
    }

    if let Some(expedited_threshold) = &gov_params.expedited_threshold {
        row.set_bigdecimal("expedited_threshold", expedited_threshold);
    }

    add_governance_parameter_derive_from(&mut row, block_number, &gov_params.block_id_last_updated);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DepositParams {
    pub min_deposit: Vec<Deposit>,
    pub max_deposit_period: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deposit {
    pub denom: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VotingParams {
    pub voting_period: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TallyParams {
    pub quorum: String,
    pub threshold: String,
    pub veto_threshold: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GovParams {
    pub deposit_params: DepositParams,
    pub voting_params: VotingParams,
    pub tally_params: TallyParams,
}
