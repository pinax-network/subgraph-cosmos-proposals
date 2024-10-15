use serde::{Deserialize, Serialize};
use serde_json::Value;
use substreams::{log, pb::substreams::Clock};
use substreams_entity_change::tables::Tables;

use crate::blocks::create_block;

pub fn push_genesis_params(tables: &mut Tables, clock: &Clock, params: &String) {
    if params.len() > 0 && clock.number == 1 {
        create_block(tables, clock);

        let parsed: GenesisParams = serde_json::from_str(&params).expect("failed to parse genesis params");
        log::debug!("GenesisParams: {:?}", parsed);

        let deposit_params = parsed.deposit_params;
        let min_deposit = deposit_params.min_deposit;
        let max_deposit_period = deposit_params.max_deposit_period;

        let voting_params = parsed.voting_params;
        let voting_period = voting_params.voting_period;

        let tally_params = parsed.tally_params;
        let quorum = tally_params.quorum;
        let threshold = tally_params.threshold;
        let veto_threshold = tally_params.veto_threshold;

        tables
            .create_row("GovernanceParameters", &clock.id)
            .set("block", &clock.id);

        create_deposit_params(tables, &clock, &min_deposit, &max_deposit_period);
        create_voting_params(tables, &clock, &voting_period);
        create_tally_params(tables, &clock, &quorum, &threshold, &veto_threshold);

        // TO-DO: to remove for GenesisParameters
        create_block(tables, &clock);
    }
}

fn create_deposit_params(tables: &mut Tables, clock: &Clock, min_deposit: &Vec<Deposit>, max_deposit_period: &str) {
    tables
        .create_row("DepositParams", &clock.id)
        .set("max_deposit_period", max_deposit_period)
        .set("block", &clock.id)
        .set("governance_parameters", &clock.id);

    for deposit in min_deposit {
        let key = format!("{}-{}", clock.id, deposit.denom);

        tables
            .create_row("MinDeposit", &key)
            .set_bigint("amount", &deposit.amount)
            .set("denom", &deposit.denom)
            .set("block", &clock.id)
            .set("deposit_params", &clock.id);
    }
}

fn create_voting_params(tables: &mut Tables, clock: &Clock, voting_period: &str) {
    tables
        .create_row("VotingParams", &clock.id)
        .set("voting_period", voting_period)
        .set("governance_parameters", &clock.id)
        .set("block", &clock.id);
}

fn create_tally_params(
    tables: &mut Tables,
    clock: &Clock,
    quorum: &String,
    threshold: &String,
    veto_threshold: &String,
) {
    tables
        .create_row("TallyParams", &clock.id)
        .set_bigdecimal("quorum", quorum)
        .set_bigdecimal("threshold", threshold)
        .set_bigdecimal("veto_threshold", veto_threshold)
        .set("governance_parameters", &clock.id)
        .set("block", &clock.id);
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
pub struct GenesisParams {
    pub deposit_params: DepositParams,
    pub voting_params: VotingParams,
    pub tally_params: TallyParams,
}
