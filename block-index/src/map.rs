use substreams_cosmos::Block;

use crate::index::{collect_event_keys, collect_transaction_keys, extract_message_type_urls, is_strict_match};

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
        let keys = collect_transaction_keys(tx_result, &block.txs[index]);

        let message_type_urls = extract_message_type_urls(&block.txs[index]);

        if is_match(keys, &params, &message_type_urls) {
            retained_indices.push(index);
        }
    }

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
        .retain(|event| is_strict_match(collect_event_keys(event), &params));

    Ok(block)
}

fn is_message_match(message_type_urls: &Vec<String>, params: &str) -> bool {
    let params_vec = parse_params(params, "message:");
    message_type_urls
        .iter()
        .any(|url| params_vec.iter().any(|param| url.starts_with(param)))
}

fn parse_params(params: &str, prefix: &str) -> Vec<String> {
    params
        .split("||")
        .filter_map(|param| {
            let trimmed = param.trim();
            if trimmed.starts_with(prefix) {
                trimmed.split_once(':').map(|(_, value)| value.trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

fn is_match(keys: Vec<String>, params: &str, message_type_urls: &Vec<String>) -> bool {
    is_strict_match(keys, &params) || is_message_match(message_type_urls, &params)
}
