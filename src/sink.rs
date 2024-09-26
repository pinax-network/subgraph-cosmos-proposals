use crate::client_update::insert_client_update_proposal;
use crate::community_pool_spends::{insert_community_pool_spend_proposal, insert_msg_community_pool_spend};
// Start of Selection
use crate::parameter_changes::insert_parameter_change_proposal;
use crate::pb::cosmos::gov::v1::MsgSubmitProposal as MsgSubmitProposalV1;
use crate::pb::cosmos::{gov::v1beta1::MsgSubmitProposal as MsgSubmitProposalV1Beta1, tx::v1beta1::Tx};
use crate::proposal_votes::push_if_proposal_votes;
use crate::serde_genesis::GenesisParams;
use crate::software_upgrades::{insert_message_software_upgrade, insert_software_upgrade_proposal};
use crate::text::insert_text_proposal;
use prost::Message;
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
        if tx_result.code != 0 {
            transactions += 1;
            continue;
        }

        let tx_hash = compute_tx_hash(&block.txs[transactions]);

        let tx_as_bytes = block.txs[transactions].as_slice();

        if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_bytes) {
            if let Some(body) = tx.body {
                for message in body.messages.iter() {
                    match message.type_url.as_str() {
                        "/cosmos.gov.v1.MsgSubmitProposal" => {
                            if let Ok(msg) = MsgSubmitProposalV1::decode(message.value.as_slice()) {
                                if push_if_message_software_upgrade(&mut tables, &msg, &tx_result, &clock, &tx_hash) {
                                    continue;
                                }
                                if push_if_msg_comm_pool_spend(&mut tables, &msg, &tx_result, &clock, &tx_hash) {
                                    continue;
                                }
                            }
                        }
                        "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
                            if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(message.value.as_slice()) {
                                if push_if_software_upgrade_prop(&mut tables, &msg, &tx_result, &clock, &tx_hash) {
                                    continue;
                                }
                                if push_if_parameter_change_prop(&mut tables, &msg, &tx_result, &clock, &tx_hash) {
                                    continue;
                                }
                                if push_if_comm_pool_spend_prop(&mut tables, &msg, &tx_result, &clock, &tx_hash) {
                                    continue;
                                }
                                if push_if_text_proposal(&mut tables, &msg, &tx_result, &clock, &tx_hash) {
                                    continue;
                                }
                                if push_if_client_update_proposal(&mut tables, &msg, &tx_result, &clock, &tx_hash) {
                                    continue;
                                }
                            }
                        }
                        "/cosmos.gov.v1beta1.MsgVote" => {
                            push_if_proposal_votes(&mut tables, message, &tx_result, &clock, &tx_hash);
                        }
                        _ => continue,
                    }
                }
            }
        }

        transactions += 1;
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
    msg_submit_proposal: &MsgSubmitProposalV1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if content.type_url == "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade" {
            insert_message_software_upgrade(tables, msg_submit_proposal, content, tx_result, clock, tx_hash);
            return true;
        }
    }
    return false;
}

pub fn push_if_software_upgrade_prop(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if content.type_url == "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" {
            insert_software_upgrade_proposal(tables, msg_submit_proposal, content, tx_result, clock, tx_hash);
            return true;
        }
    }
    return false;
}

pub fn push_if_parameter_change_prop(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if content.type_url == "/cosmos.params.v1beta1.ParameterChangeProposal" {
            insert_parameter_change_proposal(tables, msg_submit_proposal, content, tx_result, clock, tx_hash);
            return true;
        }
    }
    return false;
}

pub fn push_if_comm_pool_spend_prop(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if content.type_url == "/cosmos.distribution.v1beta1.CommunityPoolSpendProposal" {
            insert_community_pool_spend_proposal(tables, msg_submit_proposal, content, tx_result, clock, tx_hash);
            return true;
        }
    }
    return false;
}

pub fn push_if_msg_comm_pool_spend(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if content.type_url == "/cosmos.distribution.v1beta1.MsgCommunityPoolSpend" {
            insert_msg_community_pool_spend(tables, msg_submit_proposal, content, tx_result, clock, tx_hash);
            return true;
        }
    }
    return false;
}

pub fn push_if_text_proposal(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if content.type_url == "/cosmos.gov.v1beta1.TextProposal" {
            insert_text_proposal(tables, msg_submit_proposal, content, tx_result, clock, tx_hash);
            return true;
        }
    }
    return false;
}

pub fn push_if_client_update_proposal(
    tables: &mut Tables,
    msg_submit_proposal: &MsgSubmitProposalV1Beta1,
    tx_result: &TxResults,
    clock: &Clock,
    tx_hash: &str,
) -> bool {
    if let Some(content) = msg_submit_proposal.content.as_ref() {
        if content.type_url == "/ibc.core.client.v1.ClientUpdateProposal" {
            insert_client_update_proposal(tables, msg_submit_proposal, content, tx_result, clock, tx_hash);
            return true;
        }
    }
    return false;
}

pub fn code_to_string(code: u32) -> String {
    match code {
        0 => "Success".to_string(),
        2 => "Tx parse error".to_string(),
        3 => "Invalid sequence".to_string(),
        4 => "Unauthorized".to_string(),
        5 => "Insufficient funds".to_string(),
        6 => "Unknown request".to_string(),
        7 => "Invalid address".to_string(),
        8 => "Invalid pubkey".to_string(),
        9 => "Unknown address".to_string(),
        10 => "Invalid coins".to_string(),
        11 => "Out of gas".to_string(),
        12 => "Memo too large".to_string(),
        13 => "Insufficient fee".to_string(),
        14 => "Maximum number of signatures exceeded".to_string(),
        15 => "No signatures supplied".to_string(),
        16 => "Failed to marshal JSON bytes".to_string(),
        17 => "Failed to unmarshal JSON bytes".to_string(),
        18 => "Invalid request".to_string(),
        19 => "Tx already in mempool".to_string(),
        20 => "Mempool is full".to_string(),
        21 => "Tx too large".to_string(),
        22 => "Key not found".to_string(),
        23 => "Invalid account password".to_string(),
        24 => "Tx intended signer does not match the given signer".to_string(),
        25 => "Invalid gas adjustment".to_string(),
        26 => "Invalid height".to_string(),
        27 => "Invalid version".to_string(),
        28 => "Invalid chain-id".to_string(),
        29 => "Invalid type".to_string(),
        30 => "Tx timeout height".to_string(),
        31 => "Unknown extension options".to_string(),
        32 => "Incorrect account sequence".to_string(),
        33 => "Failed packing protobuf message to Any".to_string(),
        34 => "Failed unpacking protobuf message from Any".to_string(),
        35 => "Internal logic error".to_string(),
        36 => "Conflict".to_string(),
        37 => "Feature not supported".to_string(),
        38 => "Not found".to_string(),
        39 => "Internal IO error".to_string(),
        40 => "Error in app.toml".to_string(),
        41 => "Invalid gas limit".to_string(),
        42 => "Tx timeout".to_string(),
        _ => "Unknown error".to_string(),
    }
}
