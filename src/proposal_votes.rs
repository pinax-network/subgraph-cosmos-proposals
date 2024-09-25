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

        // Extract options and weights from the "option" attribute
        let options_weights = vote
            .attributes
            .iter()
            .find(|attr| attr.key == "option")
            .and_then(|attr| {
                serde_json::from_str::<Vec<serde_json::Value>>(&attr.value)
                    .ok()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|obj| {
                                let option = obj.get("option").and_then(|v| v.as_i64()).unwrap_or(0);
                                let option_str = option_number_to_string(option);
                                let weight = obj.get("weight").and_then(|v| v.as_str()).unwrap_or("0");
                                Some((option_str, weight.to_string()))
                            })
                            .collect::<Vec<(String, String)>>()
                    })
            })
            .unwrap_or(Vec::new());

        let proposal_id = vote
            .attributes
            .iter()
            .find(|attr| attr.key == "proposal_id")
            .map(|attr| attr.value.to_string())
            .unwrap_or_default();

        for (option, weight) in options_weights {
            let vote_id = format!("{}:{}:{}", tx_hash, voter, option);
            tables
                .create_row("ProposalVote", vote_id.as_str())
                .set("id", vote_id.as_str())
                .set("txHash", tx_hash)
                .set("blockNumber", clock.number)
                .set("voter", voter.as_str())
                .set("option", option.as_str())
                .set_bigdecimal("weight", &weight)
                .set("proposal", proposal_id.as_str());
        }
    }
}

pub fn option_number_to_string(option: i64) -> String {
    match option {
        1 => "Yes".to_string(),
        2 => "Abstain".to_string(),
        3 => "No".to_string(),
        4 => "NoWithVeto".to_string(),
        _ => "Unknown".to_string(),
    }
}
