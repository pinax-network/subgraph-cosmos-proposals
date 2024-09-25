use crate::parameter_changes::insert_parameter_change_proposal;
use crate::pb::cosmos::custom_proto::v1::MsgSubmitProposalNew;
use crate::pb::cosmos::{gov::v1beta1::MsgSubmitProposal, tx::v1beta1::Tx};
use crate::proposal_votes::push_if_proposal_votes;
use crate::proposals::{insert_message_software_upgrade, insert_software_upgrade_proposal};
use crate::serde_genesis::GenesisParams;
use prost_types::Any;
use sha2::{Digest, Sha256};
use substreams::{errors::Error, log, pb::substreams::Clock, Hex};
use substreams_cosmos::pb::TxResults;
use substreams_cosmos::Block;
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(params: String, clock: Clock, block: Block) -> Result<EntityChanges, Error> {
    // let parsed: GenesisParams = serde_json::from_str(&params).expect("Failed to parse params");

    let mut tables = Tables::new();

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
                            if push_if_software_upgrade_proposal(&mut tables, message, &tx_result, &clock, &tx_hash) {
                                continue;
                            }
                            if push_if_parameter_change_proposal(&mut tables, message, &tx_result, &clock, &tx_hash) {
                                continue;
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }

        transactions += 1;

        push_if_proposal_votes(&mut tables, &tx_result, &clock, &tx_hash);
    }

    let timestamp = clock.timestamp.as_ref().expect("timestamp missing").seconds;
    tables
        .create_row("Block", &clock.id)
        .set_bigint("number", &clock.number.to_string())
        .set_bigint("timestamp", &timestamp.to_string());

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
) -> bool {
    if let Ok(msg_submit_proposal) = <MsgSubmitProposalNew as prost::Message>::decode(message.value.as_slice()) {
        if let Some(content) = msg_submit_proposal.content.as_ref() {
            if content.type_url == "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade" {
                insert_message_software_upgrade(tables, msg_submit_proposal, tx_result, clock, tx_hash);
                return true;
            }
        }
    }
    return false;
}

pub fn push_if_software_upgrade_proposal(
    tables: &mut Tables,
    message: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Ok(msg_submit_proposal) = <MsgSubmitProposal as prost::Message>::decode(message.value.as_slice()) {
        if let Some(content) = msg_submit_proposal.content.as_ref() {
            if content.type_url == "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" {
                insert_software_upgrade_proposal(tables, &msg_submit_proposal, tx_result, clock, tx_hash);
                return true;
            }
        }
    }
    return false;
}

// pub fn push_if_msg_update_params(
//     tables: &mut Tables,
//     message: &Any,
//     tx_result: &TxResults,
//     clock: &Clock,
//     tx_hash: &str,
// ) -> bool {
//     if let Ok(msg_update_params) = <MsgUpdateParams as prost::Message>::decode(message.value.as_slice()) {
//         if let Some(content) = msg_update_params.content.as_ref() {
//             if content.type_url == "/cosmos.params.v1.MsgUpdateParams" {
//                 insert_msg_update_params(tables, msg_update_params, tx_result, clock, tx_hash);
//                 return true;
//             }
//         }
//     }
//     return false;
// }

pub fn push_if_parameter_change_proposal(
    tables: &mut Tables,
    message: &Any,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Ok(msg_submit_proposal) = <MsgSubmitProposal as prost::Message>::decode(message.value.as_slice()) {
        if let Some(content) = msg_submit_proposal.content.as_ref() {
            if content.type_url == "/cosmos.params.v1beta1.ParameterChangeProposal" {
                insert_parameter_change_proposal(tables, &msg_submit_proposal, tx_result, clock, tx_hash);
                return true;
            }
        }
    }
    return false;
}
