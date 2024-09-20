mod pb;
mod proposals;
use core::panic;

use crate::pb::cosmos::tx::v1beta1::Tx;
use crate::pb::custom_proto::MsgSubmitProposalNew;

use crate::pb::sf::cosmos::r#type::v2::Block;
use cosmrs::proto::Timestamp;
use pb::cosmos::gov::v1beta1::MsgSubmitProposal;
use pb::sf::substreams::v1::Clock;
use proposals::{insert_message_software_upgrade, insert_software_upgrade_proposal};
use substreams::errors::Error;

use substreams_database_change::pb::database::DatabaseChanges;

#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    let mut tables = DatabaseChanges::default();

    for (tx_as_bytes, tx_result) in block.txs.iter().zip(block.tx_results.iter()) {
        if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_bytes.as_slice()) {
            if let Some(body) = tx.body {
                for message in body.messages.iter() {
                    match message.type_url.as_str() {
                        "/cosmos.gov.v1.MsgSubmitProposal" => {
                            if let Ok(msg_submit_proposal) =
                                <MsgSubmitProposalNew as prost::Message>::decode(message.value.as_slice())
                            {
                                if let Some(content) = msg_submit_proposal.content.as_ref() {
                                    if content.type_url == "/cosmos.upgrade.v1beta1.MsgSoftwareUpgrade" {
                                        insert_message_software_upgrade(&mut tables, msg_submit_proposal, tx_result);
                                    }
                                }
                            }
                        }
                        "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
                            if let Ok(msg_submit_proposal) =
                                <MsgSubmitProposal as prost::Message>::decode(message.value.as_slice())
                            {
                                if let Some(content) = msg_submit_proposal.content.as_ref() {
                                    if content.type_url == "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" {
                                        insert_software_upgrade_proposal(&mut tables, msg_submit_proposal, tx_result);
                                    }
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
    Ok(tables)
}

// pub fn build_timestamp_string(timestamp: &Timestamp) -> String {
//     let datetime = chrono::NaiveDateTime::from_timestamp_opt(timestamp.seconds, timestamp.nanos as u32)
//         .expect("Invalid timestamp");
//     let utc_datetime = chrono::DateTime::<chrono::Utc>::from_utc(datetime, chrono::Utc);
//     utc_datetime.format("%Y-%m-%d %H:%M:%S.%3f +0000 UTC").to_string()
// }
