use prost_types::Timestamp;
use substreams::pb::substreams::Clock;
use substreams::prelude::StoreGetString;
use substreams::store::StoreGet;
use substreams_cosmos::pb::TxResults;

pub fn extract_proposal_id(
    tx_result: &substreams_cosmos::pb::TxResults,
    clock: &substreams::pb::substreams::Clock,
    tx_hash: &str,
) -> String {
    tx_result
        .events
        .iter()
        .filter(|event| event.r#type == "submit_proposal")
        .flat_map(|event| event.attributes.iter())
        .find(|attr| attr.key == "proposal_id")
        .map(|attr| attr.value.clone())
        .unwrap_or_else(|| {
            format!(
                "proposal_id not found for proposal at block {}, tx {}",
                clock.number, tx_hash
            )
        })
}

pub fn extract_proposal_id_from_tx(tx_result: &TxResults) -> Option<String> {
    tx_result
        .events
        .iter()
        .filter(|event| event.r#type == "submit_proposal")
        .flat_map(|event| event.attributes.iter())
        .find(|attr| attr.key == "proposal_id")
        .map(|attr| attr.value.clone())
}

pub fn extract_authority(tx_result: &substreams_cosmos::pb::TxResults) -> &str {
    tx_result
        .events
        .iter()
        .find(|event| event.r#type == "coin_received")
        .and_then(|event| event.attributes.iter().find(|attr| attr.key == "receiver"))
        .map(|attr| attr.value.as_str())
        .unwrap_or("")
}

pub fn to_date(clock: &Clock) -> String {
    let timestamp = clock.timestamp.as_ref().expect("missing timestamp");
    timestamp
        .to_string()
        .split('T')
        .next()
        .expect("missing date")
        .to_string()
}

pub fn get_attribute_value(event: &substreams_cosmos::pb::Event, key: &str) -> Option<String> {
    event
        .attributes
        .iter()
        .find(|attr| attr.key == key)
        .map(|attr| attr.value.clone())
}

pub fn extract_proposal_status(tx_result: &substreams_cosmos::pb::TxResults) -> &'static str {
    let voting_period_start = tx_result
        .events
        .iter()
        .filter(|event| event.r#type == "submit_proposal" || event.r#type == "proposal_deposit")
        .flat_map(|event| event.attributes.iter())
        .find(|attr| attr.key == "voting_period_start");

    if voting_period_start.is_none() {
        "DepositPeriod"
    } else {
        "VotingPeriod"
    }
}

pub fn determine_voting_end_time(
    gov_params: &GovernanceParamsStore,
    start_timestamp: &Timestamp,
    proposal_type: &str,
) -> Timestamp {
    let voting_period = if proposal_type == "Expedited" {
        &gov_params
            .expedited_voting_period
            .as_ref()
            .expect("missing expedited_voting_period")
    } else {
        &gov_params.voting_period
    };
    add_nanoseconds_to_timestamp(start_timestamp, voting_period)
}

pub fn add_nanoseconds_to_timestamp(timestamp: &Timestamp, nanoseconds: &str) -> Timestamp {
    let nanoseconds = nanoseconds.parse::<u128>().expect("Failed to parse nanoseconds");

    let total_nanos = timestamp.nanos as u128 + (nanoseconds % 1_000_000_000);
    let extra_seconds = total_nanos / 1_000_000_000;

    Timestamp {
        seconds: timestamp.seconds + (nanoseconds / 1_000_000_000) as i64 + extra_seconds as i64,
        nanos: timestamp.nanos,
    }
}

pub fn extract_gov_params(gov_params: StoreGetString) -> GovernanceParamsStore {
    let min_deposit = gov_params.get_at(0, "min_deposit").expect("missing min_deposit");
    let min_deposit_arr = build_min_deposit_array(&min_deposit);
    let expedited_min_deposit = gov_params
        .get_at(0, "expedited_min_deposit")
        .map(|deposit| build_min_deposit_array(&deposit));

    let max_deposit_period = gov_params
        .get_at(0, "max_deposit_period")
        .expect("missing max_deposit_period");
    let voting_period = gov_params.get_at(0, "voting_period").expect("missing voting_period");
    let expedited_voting_period = gov_params.get_at(0, "expedited_voting_period");
    let quorum = gov_params.get_at(0, "quorum").expect("missing quorum");
    let expedited_quorum = gov_params.get_at(0, "expedited_quorum");
    let threshold = gov_params.get_at(0, "threshold").expect("missing threshold");
    let expedited_threshold = gov_params.get_at(0, "expedited_threshold");
    let veto_threshold = gov_params.get_at(0, "veto_threshold").expect("missing veto_threshold");
    let block_id_last_updated = gov_params
        .get_at(0, "block_id_last_updated")
        .expect("missing block_id_last_updated");

    GovernanceParamsStore {
        block_id_last_updated,
        min_deposit: min_deposit_arr,
        expedited_min_deposit,
        max_deposit_period,
        voting_period,
        expedited_voting_period,
        quorum,
        expedited_quorum,
        threshold,
        expedited_threshold,
        veto_threshold,
        store_get_string: gov_params,
    }
}

pub fn get_proposal_type(gov_params: &GovernanceParamsStore, proposal_id: &str) -> String {
    gov_params
        .store_get_string
        .get_at(0, format!("proposal_id_type:{}", proposal_id))
        .expect("missing proposal_id_type")
}

fn build_min_deposit_array(min_deposit: &str) -> Vec<String> {
    serde_json::from_str::<Vec<serde_json::Value>>(min_deposit)
        .expect("invalid min_deposit format")
        .iter()
        .map(|item| {
            format!(
                "{} {}",
                item["amount"].as_str().unwrap_or_default(),
                item["denom"].as_str().unwrap_or_default()
            )
        })
        .collect()
}

pub struct GovernanceParamsStore {
    pub block_id_last_updated: String,
    pub min_deposit: Vec<String>,
    pub expedited_min_deposit: Option<Vec<String>>,
    pub max_deposit_period: String,
    pub voting_period: String,
    pub expedited_voting_period: Option<String>,
    pub quorum: String,
    pub expedited_quorum: Option<String>,
    pub threshold: String,
    pub expedited_threshold: Option<String>,
    pub veto_threshold: String,
    // StoreGetString is used to get proposal types (e.g Standard, MultipleChoice, Optimistic, Expedited)
    // Example : store.get_string(0, "proposal_id_type:234")
    pub store_get_string: StoreGetString,
}
