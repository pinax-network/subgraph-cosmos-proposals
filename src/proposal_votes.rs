use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

pub fn push_if_proposal_votes(tables: &mut Tables, tx_result: &TxResults, clock: &Clock, tx_hash: &str) {
    let proposal_votes = tx_result.events.iter().filter(|event| event.r#type == "proposal_vote");

    for vote in proposal_votes {
        let voter = vote
            .attributes
            .iter()
            .find(|attr| attr.key == "voter")
            .map(|attr| attr.value.clone())
            .unwrap_or_default();
        let option = vote
            .attributes
            .iter()
            .find(|attr| attr.key == "option")
            .map(|attr| attr.value.clone())
            .unwrap_or_default();

        let proposal_id = vote
            .attributes
            .iter()
            .find(|attr| attr.key == "proposal_id")
            .map(|attr| attr.value.to_string())
            .unwrap_or_default();

        if !voter.is_empty() && !option.is_empty() {
            let vote_id = format!("{}:{}", tx_hash, voter);
            tables
                .create_row("ProposalVote", vote_id.as_str())
                .set("id", vote_id.as_str())
                .set("txHash", tx_hash)
                .set("blockNumber", clock.number)
                .set("voter", voter.as_str())
                .set("option", option.as_str())
                .set("proposalId", proposal_id.as_str());
        }
    }
}
