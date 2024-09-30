use std::collections::HashSet;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_cosmos::Block;

#[substreams::handlers::map]
fn index_blocks(block: Block) -> Result<Keys, substreams::errors::Error> {
    let mut keys = Keys::default();

    for tx_result in block.tx_results.iter() {
        if tx_result.code != 0 {
            continue;
        }

        for event in tx_result.events.iter() {
            // Index by event type
            if event.r#type == "submit_proposal"
                || event.r#type == "proposal_vote"
                || event.r#type == "proposal_deposit"
            {
                let event_type_key = format!("event_type:{}", event.r#type);
                keys.keys.push(event_type_key);
            }
        }
    }

    Ok(keys)
}
