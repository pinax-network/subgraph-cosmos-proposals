use serde::{Deserialize, Serialize};
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::{Row, Tables};

pub fn push_genesis_params(tables: &mut Tables, clock: &Clock, params: &String) {
    if params.len() > 0 && clock.number == 1 {
        let parsed: GovParams = serde_json::from_str(&params).expect("failed to parse genesis params");
        let block_number = clock.number.to_string();
        tables
            .create_row("GovernanceParameter", &block_number)
            .set("block", &block_number);

        create_deposit_params(tables, &block_number, &parsed.deposit_params);
        create_voting_params(tables, &block_number, &parsed.voting_params);
        create_tally_params(tables, &block_number, &parsed.tally_params);
    }
}

fn add_governance_parameter_derive_from(row: &mut Row, block_number: &str) {
    row.set("block", block_number);
    row.set("governance_parameter", block_number);
}

fn create_deposit_params(tables: &mut Tables, block_number: &str, deposit_params: &DepositParams) {
    let mut min_deposit: Vec<String> = vec![];
    for deposit in &deposit_params.min_deposit {
        min_deposit.push(format! {"{} {}", deposit.amount, deposit.denom});
    }
    add_governance_parameter_derive_from(
        tables
            .create_row("DepositParam", block_number)
            .set("min_deposit", min_deposit)
            .set_bigint("max_deposit_period", &deposit_params.max_deposit_period),
        block_number,
    );
}

fn create_voting_params(tables: &mut Tables, block_number: &str, voting_params: &VotingParams) {
    add_governance_parameter_derive_from(
        tables
            .create_row("VotingParam", block_number)
            .set_bigint("voting_period", &voting_params.voting_period.to_string()),
        block_number,
    );
}

fn create_tally_params(tables: &mut Tables, block_number: &str, tally_params: &TallyParams) {
    add_governance_parameter_derive_from(
        tables
            .create_row("TallyParam", block_number)
            .set_bigdecimal("quorum", &tally_params.quorum)
            .set_bigdecimal("threshold", &tally_params.threshold)
            .set_bigdecimal("veto_threshold", &tally_params.veto_threshold),
        block_number,
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
