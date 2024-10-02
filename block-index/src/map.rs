use substreams_cosmos::Block;

#[substreams::handlers::map]
fn map_blocks(mut block: Block) -> Result<Block, substreams::errors::Error> {
    block.tx_results.retain(|tx_result| {
        if tx_result.code != 0 {
            return false;
        }
        tx_result.events.iter().any(|event| {
            event.r#type == "submit_proposal" || event.r#type == "proposal_vote" || event.r#type == "proposal_deposit"
        })
    });

    Ok(block)
}
