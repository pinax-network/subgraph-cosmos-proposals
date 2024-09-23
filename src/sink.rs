use sha2::{Digest, Sha256};
use substreams::{errors::Error, log, pb::substreams::Clock, Hex};
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let mut events = 0;
    let mut transactions = 0;

    for tx in block.tx_results {
        let tx_hash = compute_tx_hash(&block.txs[transactions]);
        for event in tx.events.iter() {
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
            .set("codespace", tx.codespace);
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
