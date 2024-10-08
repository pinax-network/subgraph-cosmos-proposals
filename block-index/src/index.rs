use std::collections::HashSet;
use substreams::{matches_keys_in_parsed_expr, pb::sf::substreams::index::v1::Keys};
use substreams_cosmos::{pb::TxResults, Block};

#[substreams::handlers::map]
fn index_blocks(block: Block) -> Result<Keys, substreams::errors::Error> {
    let mut keys = HashSet::new();

    for tx_result in block.tx_results.iter() {
        keys.extend(collect_transaction_keys(tx_result));
    }

    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}

pub fn collect_transaction_keys(tx_result: &TxResults) -> Vec<String> {
    let mut keys = Vec::new();

    for event in tx_result.events.iter() {
        keys.push(format!("type:{}", event.r#type));
        event.attributes.iter().for_each(|attr| {
            keys.push(format!("attr:{}", attr.key));
        });
    }
    keys
}

pub fn is_match(query: Vec<String>, params: &str) -> bool {
    // match all if wildcard is used
    // `eosio:onblock` actions are excluded from wildcard
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
