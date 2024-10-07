use substreams_cosmos::Block;

#[substreams::handlers::map]
fn map_blocks(params: String, mut block: Block) -> Result<Block, substreams::errors::Error> {
    // TO-DO: apply retain also on `txs`
    // https://github.com/pinax-network/subgraph-cosmos-proposals/issues/8
    block.tx_results.retain(|tx_result| {
        if tx_result.code != 0 {
            return false;
        }
        // TO-DO: apply retain using `params`
        // https://github.com/pinax-network/subgraph-cosmos-proposals/issues/4
        tx_result.events.iter().any(|event| {
            event.r#type == "submit_proposal" || event.r#type == "proposal_vote" || event.r#type == "proposal_deposit"
        })
        // TO-DO: Apply filtering based on `events.attr`
        // https://github.com/pinax-network/subgraph-cosmos-proposals/issues/9
    });

    Ok(block)
}
