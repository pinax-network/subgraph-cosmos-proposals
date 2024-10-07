use substreams_entity_change::tables::Tables;

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

pub fn insert_proposal_entity(
    tables: &mut Tables,
    id: &str,
    tx_hash: &str,
    block: &str,
    proposal_type: &str,
    proposer: &str,
    authority: &str,
    title: &str,
    description: &str,
    metadata: &str,
) {
    tables
        .create_row("Proposal", id)
        .set("txHash", tx_hash)
        .set("block", block)
        .set("proposalType", proposal_type)
        .set("proposer", proposer)
        .set("authority", authority)
        .set("title", title)
        .set("description", description)
        .set("metadata", metadata);
}

pub fn insert_content_entity_json(tables: &mut Tables, id: &str, type_url: &str, json_data: &str) {
    tables
        .create_row("Content", id)
        .set("typeUrl", type_url)
        .set("jsonData", json_data)
        .set("proposal", id);
}

pub fn insert_content_entity_raw_data(tables: &mut Tables, id: &str, type_url: &str, value: &str) {
    tables
        .create_row("Content", id)
        .set("typeUrl", type_url)
        .set("value", value)
        .set("proposal", id);
}
