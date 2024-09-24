use crate::pb::cosmos::custom_proto::v1::MsgSubmitProposalNew;
use crate::pb::cosmos::{gov::v1beta1::MsgSubmitProposal, tx::v1beta1::Tx};
use crate::proposals::{insert_message_software_upgrade, insert_software_upgrade_proposal};
use prost_types::Any;
use sha2::{Digest, Sha256};
use substreams::{errors::Error, log, pb::substreams::Clock, Hex};
use substreams_cosmos::pb::TxResults;
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let mut events = 0;
    let mut transactions = 0;

    for tx_result in block.tx_results {
        let tx_hash = compute_tx_hash(&block.txs[transactions]);

        let tx_as_bytes = block.txs[transactions].as_slice();

        if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_bytes) {
            if let Some(body) = tx.body {
                for message in body.messages.iter() {
                    match message.type_url.as_str() {
                        "/cosmos.gov.v1.MsgSubmitProposal" => {
                            push_if_message_software_upgrade(&mut tables, message, &tx_result, &clock, &tx_hash);
                        }
                        "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
                            push_if_software_upgrade_proposal(&mut tables, message, &tx_result, &clock, &tx_hash);
                        }
                        _ => continue,
                    }
                }
            }
        }

        push_if_proposal_votes(&mut tables, &tx_result, &clock, &tx_hash);
    }

    log::debug!("Processed transactions {} & events {}", transactions, events);

    // let timestamp = clock.timestamp.as_ref().expect("timestamp missing").seconds;
    // tables
    //     .create_row("Block", &clock.id)
    //     .set_bigint("number", &clock.number.to_string())
    //     .set_bigint("timestamp", &timestamp.to_string());

    Ok(tables.to_entity_changes())
}

// Should be included in Substreams Cosmos
fn compute_tx_hash(tx_as_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(tx_as_bytes);
    let tx_hash = hasher.finalize();
    return Hex::encode(tx_hash);
}

pub fn push_if_message_software_upgrade(
    tables: &mut Tables,
    message: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(msg_submit_proposal) = <MsgSubmitProposalNew as prost::Message>::decode(message.value.as_slice()) {
        if let Some(content) = msg_submit_proposal.content.as_ref() {
            if content.type_url == "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade" {
                insert_message_software_upgrade(tables, msg_submit_proposal, tx_result, clock, tx_hash);
            }
        }
    }
}

pub fn push_if_software_upgrade_proposal(
    tables: &mut Tables,
    message: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) {
    if let Ok(msg_submit_proposal) = <MsgSubmitProposal as prost::Message>::decode(message.value.as_slice()) {
        if let Some(content) = msg_submit_proposal.content.as_ref() {
            if content.type_url == "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" {
                insert_software_upgrade_proposal(tables, msg_submit_proposal, tx_result, clock, tx_hash);
            }
        }
    }
}

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

        log::debug!("voter: {}", voter);
        log::debug!("option: {}", option);
        // log::debug!("proposal_id: {}", proposal_id);

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
