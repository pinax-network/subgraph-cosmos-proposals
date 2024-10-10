use serde::{Deserialize, Serialize};
use substreams::{log, pb::substreams::Clock};
use substreams_entity_change::tables::Tables;

pub fn push_genesis_params(params: &String, clock: &Clock, _tables: &mut Tables) {
    let parsed: GenesisParams = serde_json::from_str(&params).expect("Failed to parse params");
    if clock.number == 1 {
        log::debug!("GenesisParams: {:?}", parsed);
    }
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
