use std::collections::HashSet;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_cosmos::Block;

#[substreams::handlers::map]
fn index_blocks(block: Block) -> Result<Keys, substreams::errors::Error> {
    let mut keys = HashSet::new();

    for tx_result in block.tx_results.iter() {
        for event in tx_result.events.iter() {
            keys.insert(format!("type:{}", event.r#type));
            event.attributes.iter().for_each(|attr| {
                keys.insert(format!("attr:{}", attr.key));
            });
        }
    }
    Ok(Keys {
        keys: keys.into_iter().collect(),
    })
}
