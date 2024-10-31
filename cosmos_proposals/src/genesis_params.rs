use serde::{Deserialize, Serialize};
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::{Row, Tables};

use crate::utils::GovernanceParamsFlat;

pub fn push_gov_params(tables: &mut Tables, clock: &Clock, gov_params: &GovernanceParamsFlat) {
    let block_number = clock.number.to_string();

    tables
        .create_row("GovernanceParameter", &gov_params.hashed_id)
        .set("id", &gov_params.hashed_id)
        .set("block", &block_number);

    create_deposit_params(tables, &block_number, gov_params);
    create_voting_params(tables, &block_number, gov_params);
    create_tally_params(tables, &block_number, gov_params);
}

fn add_governance_parameter_derive_from(row: &mut Row, block_number: &str, id: &str) {
    row.set("block", block_number).set("governance_parameter", id);
}

fn create_deposit_params(tables: &mut Tables, block_number: &str, gov_params: &GovernanceParamsFlat) {
    add_governance_parameter_derive_from(
        tables
            .create_row("DepositParam", &gov_params.hashed_id)
            .set("min_deposit", &gov_params.min_deposit)
            .set_bigint("max_deposit_period", &gov_params.max_deposit_period),
        &block_number,
        &gov_params.hashed_id,
    );
}

fn create_voting_params(tables: &mut Tables, block_number: &str, gov_params: &GovernanceParamsFlat) {
    add_governance_parameter_derive_from(
        tables
            .create_row("VotingParam", &gov_params.hashed_id)
            .set_bigint("voting_period", &gov_params.voting_period),
        &block_number,
        &gov_params.hashed_id,
    );
}

fn create_tally_params(tables: &mut Tables, block_number: &str, gov_params: &GovernanceParamsFlat) {
    add_governance_parameter_derive_from(
        tables
            .create_row("TallyParam", &gov_params.hashed_id)
            .set_bigdecimal("quorum", &gov_params.quorum)
            .set_bigdecimal("threshold", &gov_params.threshold)
            .set_bigdecimal("veto_threshold", &gov_params.veto_threshold),
        &block_number,
        &gov_params.hashed_id,
    );
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
