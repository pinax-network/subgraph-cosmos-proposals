use crate::pb::cosmos::tx::v1beta1::Tx;
use crate::utils::GovernanceParamsStore;
use base64::prelude::*;
use sha2::{Digest, Sha256};
use substreams::pb::substreams::Clock;
use substreams::Hex;
use substreams_cosmos::pb::TxResults;
use substreams_cosmos::Block;
use substreams_entity_change::tables::Tables;

use crate::messages::push_messages;

pub fn push_transactions(block: &Block, tables: &mut Tables, clock: &Clock, gov_params: &GovernanceParamsStore) {
    for (i, tx_result) in block.tx_results.iter().enumerate() {
        let tx_hash = compute_tx_hash(&block.txs[i]);
        let tx_as_bytes = block.txs[i].as_slice();

        let tx_as_bytes = tx_as_bytes;

        if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_bytes) {
            create_transaction(tables, tx_result, clock, &tx_hash, &tx);
            if let Some(body) = tx.body {
                push_messages(tables, tx_result, clock, &tx_hash, &body.messages, &gov_params);
            }
        }
    }
}

fn create_transaction(tables: &mut Tables, tx_result: &TxResults, clock: &Clock, tx_hash: &str, tx: &Tx) {
    // TODO: handle auth_info

    tables
        .create_row("Transaction", tx_hash)
        .set("codespace", &tx_result.codespace)
        .set("code", tx_result.code)
        .set("gas_used", tx_result.gas_used)
        .set("gas_wanted", tx_result.gas_wanted)
        .set("info", tx_result.info.as_str())
        .set("log", tx_result.log.as_str())
        .set("block", &clock.id);

    create_signatures(tables, tx_hash, tx);
}

fn create_signatures(tables: &mut Tables, tx_hash: &str, tx: &Tx) {
    for (index, signature) in tx.signatures.iter().enumerate() {
        let signature_id = format!("{}-{}", tx_hash, index);
        let encoded = BASE64_STANDARD.encode(signature.as_slice());
        tables
            .create_row("Signature", &signature_id)
            .set("signature", encoded)
            .set("transaction", tx_hash);
    }
}

pub fn _code_to_string(code: u32) -> String {
    match code {
        0 => "Success".to_string(),
        2 => "Tx parse error".to_string(),
        3 => "Invalid sequence".to_string(),
        4 => "Unauthorized".to_string(),
        5 => "Insufficient funds".to_string(),
        6 => "Unknown request".to_string(),
        7 => "Invalid address".to_string(),
        8 => "Invalid pubkey".to_string(),
        9 => "Unknown address".to_string(),
        10 => "Invalid coins".to_string(),
        11 => "Out of gas".to_string(),
        12 => "Memo too large".to_string(),
        13 => "Insufficient fee".to_string(),
        14 => "Maximum number of signatures exceeded".to_string(),
        15 => "No signatures supplied".to_string(),
        16 => "Failed to marshal JSON bytes".to_string(),
        17 => "Failed to unmarshal JSON bytes".to_string(),
        18 => "Invalid request".to_string(),
        19 => "Tx already in mempool".to_string(),
        20 => "Mempool is full".to_string(),
        21 => "Tx too large".to_string(),
        22 => "Key not found".to_string(),
        23 => "Invalid account password".to_string(),
        24 => "Tx intended signer does not match the given signer".to_string(),
        25 => "Invalid gas adjustment".to_string(),
        26 => "Invalid height".to_string(),
        27 => "Invalid version".to_string(),
        28 => "Invalid chain-id".to_string(),
        29 => "Invalid type".to_string(),
        30 => "Tx timeout height".to_string(),
        31 => "Unknown extension options".to_string(),
        32 => "Incorrect account sequence".to_string(),
        33 => "Failed packing protobuf message to Any".to_string(),
        34 => "Failed unpacking protobuf message from Any".to_string(),
        35 => "Internal logic error".to_string(),
        36 => "Conflict".to_string(),
        37 => "Feature not supported".to_string(),
        38 => "Not found".to_string(),
        39 => "Internal IO error".to_string(),
        40 => "Error in app.toml".to_string(),
        41 => "Invalid gas limit".to_string(),
        42 => "Tx timeout".to_string(),
        _ => "Unknown error".to_string(),
    }
}

// Should be included in Substreams Cosmos
pub fn compute_tx_hash(tx_as_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx_as_bytes);
    let tx_hash = hasher.finalize();
    return Hex::encode(tx_hash);
}
