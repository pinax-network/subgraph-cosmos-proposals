use std::collections::HashSet;
use substreams::{
    matches_keys_in_parsed_expr,
    pb::{sf::substreams::index::v1::Keys, substreams::Clock},
};
use substreams_cosmos::{
    pb::{Event, TxResults},
    Block,
};

use crate::protobufs::PartialTx;

#[substreams::handlers::map]
fn index_blocks(block: Block, clock: Clock) -> Result<Keys, substreams::errors::Error> {
    let mut keys = HashSet::new();

    keys.insert(format!("block.number:{}", clock.number));

    for event in block.events.iter() {
        keys.extend(collect_event_keys(event));
    }

    for (index, tx_result) in block.tx_results.iter().enumerate() {
        keys.extend(collect_transaction_keys(tx_result, &block.txs[index]));
    }

    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}

pub fn collect_transaction_keys(tx_result: &TxResults, tx_as_bytes: &[u8]) -> Vec<String> {
    let mut keys = Vec::new();

    for event in tx_result.events.iter() {
        keys.extend(collect_event_keys(event));
    }

    let msg_types = extract_message_types(tx_as_bytes);
    let msg_types_expanded = expand_match_variations(msg_types, ".");

    keys.extend(msg_types_expanded);

    keys
}

pub fn collect_event_keys(event: &Event) -> Vec<String> {
    let mut keys = Vec::new();

    keys.push(format!("type:{}", event.r#type));
    event.attributes.iter().for_each(|attr| {
        keys.push(format!("attr:{}", attr.key));
    });
    keys
}

pub fn is_match(query: Vec<String>, params: &str) -> bool {
    // match all if wildcard is used
    if query.len() > 0 && params == "*" {
        return true;
    }
    match matches_keys_in_parsed_expr(&query, &params) {
        Ok(true) => {
            return true;
        }
        Ok(false) => {
            return false;
        }
        Err(e) => {
            panic!("Error: {:?}", e);
        }
    };
}

pub fn extract_message_types(tx_as_bytes: &[u8]) -> Vec<String> {
    let mut msg_types = Vec::new();

    if let Ok(tx) = <PartialTx as prost::Message>::decode(tx_as_bytes) {
        if let Some(body) = tx.body {
            for message in body.messages.iter() {
                // Remove the leading '/' from the type URL
                let msg_type = &message.type_url[1..];
                msg_types.push(format!("message:{}", msg_type));
            }
        }
    }
    msg_types
}

fn expand_match_variations(strings: Vec<String>, delimiter: &str) -> Vec<String> {
    let mut result = Vec::new();
    for s in strings {
        let parts: Vec<&str> = s.split(delimiter).collect();
        let mut current = String::new();
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                current.push_str(delimiter);
            }
            current.push_str(part);
            result.push(current.clone());
        }
    }
    result
}
