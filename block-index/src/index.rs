use std::collections::HashSet;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_cosmos::Block;

#[substreams::handlers::map]
fn index_blocks(block: Block) -> Result<Keys, substreams::errors::Error> {
    let mut keys = HashSet::new();

    for tx_result in block.tx_results.iter() {
        // only index successful transactions
        if tx_result.code != 0 {
            continue;
        }

        for event in tx_result.events.iter() {
            event.attributes.iter().for_each(|attr| {
                keys.insert(format!("event.attribute:{}", attr.key));
                keys.insert(format!("event.type:{}:{}", event.r#type, attr.key));
            });
            keys.insert(format!("event.type:{}", event.r#type));
        }
    }
    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}
