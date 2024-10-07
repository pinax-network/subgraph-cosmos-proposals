use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_cosmos::Block;

#[substreams::handlers::map]
fn index_blocks(block: Block) -> Result<Keys, substreams::errors::Error> {
    let mut keys = Keys::default();

    for tx_result in block.tx_results.iter() {
        // only index successful transactions
        if tx_result.code != 0 {
            continue;
        }

        for event in tx_result.events.iter() {
            event.attributes.iter().for_each(|attr| {
                keys.keys.push(format!("event.attribute:{}", attr.key));
                keys.keys.push(format!("event.attribute:{}:{}", attr.key, attr.value));
                keys.keys.push(format!("event.type:{}:{}", event.r#type, attr.key));
                keys.keys
                    .push(format!("event.type:{}:{}:{}", event.r#type, attr.key, attr.value));
            });
            keys.keys.push(format!("event.type:{}", event.r#type));
        }
    }

    Ok(keys)
}
