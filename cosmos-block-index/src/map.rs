use substreams_cosmos::Block;

use crate::index::{collect_event_keys, collect_transaction_keys, is_match};

#[substreams::handlers::map]
fn map_blocks(params: String, mut block: Block) -> Result<Block, substreams::errors::Error> {
    substreams::log::debug!("map_blocks: {:?}", params);
    // Filter both tx_results and txs based on the same criteria
    let retained_indices: Vec<usize> = block
        .tx_results
        .iter()
        .enumerate()
        .filter_map(|(index, tx_result)| {
            if tx_result.code == 0 {
                let keys = collect_transaction_keys(tx_result, &block.txs[index]);
                if is_match(keys, &params) {
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Retain only the tx_results at the retained indices
    block.tx_results = block
        .tx_results
        .into_iter()
        .enumerate()
        .filter(|(index, _)| retained_indices.contains(index))
        .map(|(_, tx)| tx)
        .collect();

    // Retain only the txs at the retained indices
    block.txs = block
        .txs
        .into_iter()
        .enumerate()
        .filter(|(index, _)| retained_indices.contains(index))
        .map(|(_, tx)| tx)
        .collect();

    // Retain block events based on params
    block
        .events
        .retain(|event| is_match(collect_event_keys(event), &params));

    Ok(block)
}
