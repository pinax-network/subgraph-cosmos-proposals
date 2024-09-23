use crate::pb::cosmos::custom_proto::v1::MsgSubmitProposalNew;
use crate::pb::cosmos::upgrade::v1beta1::SoftwareUpgradeProposal;
use crate::pb::cosmos::{gov::v1beta1::MsgSubmitProposal, tx::v1beta1::Tx};
use crate::proposals::{insert_message_software_upgrade, insert_software_upgrade_proposal};
use prost_types::Any;
use sha2::{Digest, Sha256};
use substreams::{errors::Error, log, pb::substreams::Clock, Hex};
use substreams_cosmos::pb::TxResults;
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let mut events = 0;
    let mut transactions = 0;

    for tx_result in block.tx_results {
        let tx_hash = compute_tx_hash(&block.txs[transactions]);

        let tx_as_bytes = block.txs[transactions].as_slice();

        if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_bytes) {
            if let Some(body) = tx.body {
                for message in body.messages.iter() {
                    match message.type_url.as_str() {
                        "/cosmos.gov.v1.MsgSubmitProposal" => {
                            push_if_message_software_upgrade(&mut tables, message, &tx_result, &clock, &tx_hash);
                        }
                        "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
                            push_if_software_upgrade_proposal(&mut tables, message, &tx_result, &clock, &tx_hash);
                        }
                        _ => continue,
                    }
                }
            }
        }

        let msg_votes = tx_result.events.iter().filter(|event| event.r#type == "message_vote");

        for event in tx_result.events.iter() {
            let event_key = format!("{}:{}", tx_hash, events);
            tables
                .create_row("Event", event_key.as_str())
                // derive from
                .set("block", &clock.id)
                .set("transaction", tx_hash.clone())
                // event
                .set("type", &event.r#type);
            events += 1;

            for attribute in event.attributes.iter() {
                let attribute_key = format!("{}:{}:{}", tx_hash, events, attribute.key);
                tables
                    .create_row("Attribute", attribute_key)
                    // derive from
                    .set("block", &clock.id)
                    .set("transaction", &tx_hash)
                    .set("event", &event_key)
                    // attribute
                    .set("key", attribute.key.clone())
                    .set("value", attribute.value.clone());
            }
        }

        tables
            .create_row("Transaction", tx_hash)
            // derive From
            .set("block", &clock.id)
            // transaction
            .set("codespace", tx_result.codespace);
        transactions += 1;
    }

    log::debug!("Processed transactions {} & events {}", transactions, events);

    let timestamp = clock.timestamp.as_ref().expect("timestamp missing").seconds;
    tables
        .create_row("Block", &clock.id)
        .set_bigint("number", &clock.number.to_string())
        .set_bigint("timestamp", &timestamp.to_string());

    Ok(tables.to_entity_changes())
}

// Should be included in Substreams Cosmos
fn compute_tx_hash(tx_as_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx_as_bytes);
    let tx_hash = hasher.finalize();
    return Hex::encode(tx_hash);
}

pub fn push_if_message_software_upgrade(
    tables: &mut Tables,
    message: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(msg_submit_proposal) = <MsgSubmitProposalNew as prost::Message>::decode(message.value.as_slice()) {
        if let Some(content) = msg_submit_proposal.content.as_ref() {
            if content.type_url == "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade" {
                insert_message_software_upgrade(tables, msg_submit_proposal, tx_result, clock, tx_hash);
            }
        }
    }
}

pub fn push_if_software_upgrade_proposal(
    tables: &mut Tables,
    message: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(msg_submit_proposal) = <MsgSubmitProposal as prost::Message>::decode(message.value.as_slice()) {
        if let Some(content) = msg_submit_proposal.content.as_ref() {
            if content.type_url == "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" {
                insert_software_upgrade_proposal(tables, msg_submit_proposal, tx_result, clock, tx_hash);
            }
        }
    }
}
