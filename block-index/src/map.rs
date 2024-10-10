use substreams_cosmos::Block;

use crate::index::{collect_event_keys, collect_transaction_keys, extract_message_types, is_strict_match};

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
                let message_types = extract_message_types(&block.txs[index]);
                if is_match(keys, &params, &message_types) {
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
        .retain(|event| is_strict_match(collect_event_keys(event), &params));

    Ok(block)
}

fn is_match(keys: Vec<String>, params: &str, message_types: &[String]) -> bool {
    is_strict_match(keys, &params) || is_message_match(message_types, &params)
}

fn is_message_match(message_types: &[String], params: &str) -> bool {
    let params_2d = parse_message_params(params);
    let message_types_2d = to_2d_string_vec(message_types, ".");

    substreams::log::debug!("Parsed Params 2D: {:?}", params_2d);
    substreams::log::debug!("Parsed Message Types 2D: {:?}", message_types_2d);

    params_2d.iter().any(|param_vec| {
        message_types_2d.iter().any(|message_vec| {
            param_vec.iter().enumerate().all(|(i, part)| {
                message_vec.get(i).map_or(false, |msg| {
                    substreams::log::debug!("is_message_match: {:?} {:?}", msg, part);
                    msg == part
                })
            })
        })
    })
}

fn to_2d_string_vec(str_slice: &[String], delimiter: &str) -> Vec<Vec<String>> {
    str_slice
        .iter()
        .map(|s| s.split(delimiter).map(String::from).collect())
        .collect()
}

// Parse message params from the format "message:type.type2 || message:type.type2"
// Returns a 2D vector of strings, where each inner vector represents a message type
// and each string represents a part of the message type
fn parse_message_params(params: &str) -> Vec<Vec<String>> {
    params
        .split("||")
        .filter_map(|param| {
            let trimmed = param.trim();
            if trimmed.starts_with("message:") {
                Some(trimmed.split('.').map(String::from).collect())
            } else {
                None
            }
        })
        .collect()
}
