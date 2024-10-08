use substreams_cosmos::Block;

use crate::index::{collect_event_keys, collect_transaction_keys, is_match};

#[substreams::handlers::map]
fn map_blocks(params: String, mut block: Block) -> Result<Block, substreams::errors::Error> {
    // Filter both tx_results and txs based on the same criteria
    let mut retained_indices = Vec::new();

    for (index, tx_result) in block.tx_results.iter().enumerate() {
        // only index successful transactions
        if tx_result.code != 0 {
            continue;
        };
        // transaction keys must match event type or attributes based on provided params
        let keys = collect_transaction_keys(tx_result);
        if is_match(keys, &params) {
            retained_indices.push(index);
        }
    }

    // Retain only the tx_results at the retained indices
    block.tx_results = block
        .tx_results
        .into_iter()
        .enumerate()
        .filter(|(index, _)| retained_indices.contains(index))
        .map(|(_, tx_result)| tx_result)
        .collect();

    // Retain only the txs at the retained indices
    block.txs = block
        .txs
        .into_iter()
        .enumerate()
        .filter(|(index, _)| retained_indices.contains(index))
        .map(|(_, tx)| tx)
        .collect();

    // // remove events from tx_results
    // block
    //     .events
    //     .retain(|event| is_match(collect_event_keys(event), &params));

    Ok(block)
}
