use substreams::pb::substreams::Clock;

use crate::pb::cosmos::base::v1beta1::Coin;

pub fn extract_initial_deposit(initial_deposit: &[Coin]) -> (&str, &str) {
    initial_deposit
        .get(0)
        .map_or(("", "0"), |deposit| (deposit.denom.as_str(), deposit.amount.as_str()))
}

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
