use substreams_cosmos::Block;

#[substreams::handlers::map]
fn map_blocks(params: String, mut block: Block) -> Result<Block, substreams::errors::Error> {
    // Filter both tx_results and txs based on the same criteria
    let mut retained_indices = Vec::new();

    for (index, tx_result) in block.tx_results.iter().enumerate() {
        if tx_result.code == 0
            && tx_result.events.iter().any(|event| {
                event.r#type == "submit_proposal"
                    || event.r#type == "proposal_vote"
                    || event.r#type == "proposal_deposit"
            })
        {
            retained_indices.push(index);
        }
    }

    // Retain tx_results
    block.tx_results = retained_indices
        .iter()
        .map(|&index| block.tx_results[index].clone())
        .collect();

    // Retain txs
    block.txs = retained_indices.iter().map(|&index| block.txs[index].clone()).collect();

    // TO-DO: apply retain using `params`
    // https://github.com/pinax-network/subgraph-cosmos-proposals/issues/4

    // TO-DO: Apply filtering based on `events.attr`
    // https://github.com/pinax-network/subgraph-cosmos-proposals/issues/9

    Ok(block)
}
