use prost::Message;
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::{order_by::insert_order_by, pb::cosmos::gov::v1beta1::MsgVote};

pub fn push_proposal_vote(tables: &mut Tables, msg: &Any, tx_result: &TxResults, clock: &Clock, tx_hash: &str) {
    let proposal_votes = tx_result.events.iter().filter(|event| event.r#type == "proposal_vote");

    for vote in proposal_votes {
        let voter = vote
            .attributes
            .iter()
            .find(|attr| attr.key == "voter")
            .map(|attr| attr.value.clone())
            .unwrap_or_else(|| {
                MsgVote::decode(msg.value.as_slice())
                    .map(|vote_msg| vote_msg.voter)
                    .unwrap_or_default()
            });

        let proposal_id = vote
            .attributes
            .iter()
            .find(|attr| attr.key == "proposal_id")
            .map(|attr| attr.value.to_string())
            .expect(&format!(
                "Proposal_id not found for proposal vote at block {}, tx {}",
                clock.number, tx_hash
            ));

        // Extract options and weights from the "option" attribute
        // Votes can take three forms:
        // 1. JSON array of objects with "option" and "weight" fields
        // 2. JSON object with "option" and "weight" fields
        // 3. Key-value string with "option" and "weight" fields
        let options_weights = vote
            .attributes
            .iter()
            .find(|attr| attr.key == "option")
            .and_then(|attr| {
                let value = &attr.value;
                // Attempt to parse as JSON
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(value) {
                    // If parsed as JSON, handle both array and object cases
                    let items = match parsed {
                        serde_json::Value::Array(arr) => arr,         // If it's an array, use it directly
                        serde_json::Value::Object(_) => vec![parsed], // If it's an object, wrap it in a vector
                        _ => return None,
                    };

                    Some(
                        items
                            .iter()
                            .filter_map(|obj| {
                                // Extract option and weight from each item
                                let option = obj.get("option").and_then(|v| v.as_i64()).unwrap_or(0);
                                let option_str = option_number_to_proper_string(option);
                                let weight = obj.get("weight").and_then(|v| v.as_str()).unwrap_or("0");
                                Some((option_str, weight.to_string()))
                            })
                            .collect::<Vec<(String, String)>>(),
                    )
                } else {
                    // Attempt to parse as key-value string
                    let parts: Vec<&str> = value.split_whitespace().collect();
                    let mut option_str = "Unknown".to_string();
                    let mut weight = "0".to_string();
                    for part in parts {
                        if part.starts_with("option:") {
                            option_str = part.trim_start_matches("option:").to_string();
                            option_str = option_string_to_proper_string(option_str.as_str());
                        } else if part.starts_with("weight:\"") && part.ends_with("\"") {
                            weight = part.trim_start_matches("weight:\"").trim_end_matches("\"").to_string();
                        }
                    }
                    Some(vec![(option_str, weight)])
                }
            })
            .expect("Failed to parse options and weights for the proposal vote");

        for (option, weight) in options_weights {
            let vote_id = format!("{}:{}:{}", tx_hash, &proposal_id, option);
            let row = tables
                .create_row("Vote", &vote_id)
                .set("txHash", tx_hash)
                .set("voter", &voter)
                .set("option", &option)
                .set_bigdecimal("weight", &weight)
                .set("proposal", &proposal_id);

            insert_order_by(row, clock);
        }
    }
}

fn option_number_to_proper_string(option: i64) -> String {
    match option {
        1 => "Yes".to_string(),
        2 => "Abstain".to_string(),
        3 => "No".to_string(),
        4 => "NoWithVeto".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn option_string_to_proper_string(option: &str) -> String {
    match option {
        "VOTE_OPTION_YES" => "Yes".to_string(),
        "VOTE_OPTION_ABSTAIN" => "Abstain".to_string(),
        "VOTE_OPTION_NO" => "No".to_string(),
        "VOTE_OPTION_NO_WITH_VETO" => "NoWithVeto".to_string(),
        _ => "Unknown".to_string(),
    }
}
