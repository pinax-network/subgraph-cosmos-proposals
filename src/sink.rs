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
            let attributes: Vec<String> = event
                .attributes
                .iter()
                .map(|attr| format!("key={} value={}", attr.key, attr.value))
                .collect();

            let key = format!("{}:{}", tx_hash, events);
            tables
                .create_row("events", key)
                .set_bigint("block_number", &clock.number.to_string())
                .set("tx_hash", tx_hash.clone())
                .set("attributes", attributes);
            events += 1;
        }
        transactions += 1;
    }

    log::debug!("Processed transactions {} & events {}", transactions, events);

    Ok(tables.to_entity_changes())
}

// Should be included in Substreams Cosmos
fn compute_tx_hash(tx_as_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx_as_bytes);
    let tx_hash = hasher.finalize();
    return Hex::encode(tx_hash);
}
