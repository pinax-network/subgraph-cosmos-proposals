use crate::{pb::cosmos::tx::v1beta1::Tx, proposals::proposals::handle_proposals};
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

pub fn push_messages(tables: &mut Tables, tx_result: &TxResults, clock: &Clock, tx_hash: &str, tx_as_bytes: &[u8]) {
    if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_bytes) {
        if let Some(body) = tx.body {
            for message in body.messages.iter() {
                handle_proposals(message, tables, tx_result, clock, tx_hash);
            }
        }
    }
}
