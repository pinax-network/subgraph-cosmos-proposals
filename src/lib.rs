mod pb;
use core::panic;
use std::collections::HashMap;

use crate::pb::cosmos::tx::v1beta1::Tx;
use crate::pb::custom_proto::{MsgSoftwareUpgrade, MsgSubmitProposal};
use crate::pb::sf::cosmos::r#type::v2::Block;
use pb::sf::substreams::v1::Clock;
use prost_types::Any;
use sha2::{Digest, Sha256};
use substreams::errors::Error;
use substreams::log;
use substreams::matches_keys_in_parsed_expr;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams_database_change::pb::database::DatabaseChanges;

// pub fn all_transactions(block: Block) -> Result<TransactionList, Error> {
//     // Mutable list to add the output of the Substreams
//     let mut transactions: Vec<Transaction> = Vec::new();

//     if block.txs.len() != block.tx_results.len() {
//         return Err(anyhow!("Transaction list and result list do not match"));
//     }

//     for i in 0..block.txs.len() {
//         let tx_as_bytes = block.txs.get(i).unwrap();
//         let tx_as_u8 = &tx_as_bytes[..];

//         let tx_result = block.tx_results.get(i).unwrap();
//         // substreams::log::println("00-----------------");

//         if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_u8) {
//             if let Some(body) = tx.body {
//                 if body
//                     .messages
//                     .iter()
//                     .any(|message| message.type_url == "/cosmos.gov.v1beta1.MsgSubmitProposal")
//                 {
//                     let transaction_memo = body.memo;
//                     let transaction_timeout_height = body.timeout_height;
//                     let transaction_extension_options = body.extension_options;
//                     let transaction_non_critical_extension_options = body.non_critical_extension_options;
//                     // substreams::log::println("A-----------------");

//                     let messages = Vec::new();
//                     // substreams::log::println("I-----------------");
//                     // substreams::log::println(messages.len().to_string());

//                     let transaction = Transaction {
//                         raw_tx: tx_as_bytes.to_vec(),
//                         hash: compute_tx_hash(tx_as_bytes),
//                         memo: transaction_memo,
//                         messages: messages,
//                         timeout_height: transaction_timeout_height,
//                         extension_options: transaction_extension_options,
//                         non_critical_extension_options: transaction_non_critical_extension_options,
//                         result_code: tx_result.code,
//                         result_data: tx_result.data.to_vec(),
//                         result_log: tx_result.log.to_string(),
//                         result_info: tx_result.info.to_string(),
//                         result_gas_wanted: tx_result.gas_wanted,
//                         result_gas_used: tx_result.gas_used,
//                         result_events: tx_result.events.to_vec(),
//                         result_codespace: tx_result.codespace.to_string(),
//                         auth_info: tx.auth_info,
//                         signatures: tx.signatures,
//                     };
//                     transactions.push(transaction);
//                 }
//             }
//         }
//     }

//     Ok(TransactionList {
//         transactions: transactions,
//         clock: Some(Clock {
//             id: hex::encode(block.hash),
//             number: block.height as u64,
//             timestamp: block.time,
//         }),
//     })
// }

// #[substreams::handlers::map]
// pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
//     let mut tables: DatabaseChanges = DatabaseChanges::default();

//     for tx_as_bytes in block.txs.iter() {
//         let tx_as_u8 = tx_as_bytes.as_slice();

//         if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_u8) {
//             if let Some(body) = tx.body {
//                 if let Some(message) = body
//                     .messages
//                     .iter()
//                     .find(|message| message.type_url == "/cosmos.gov.v1beta1.MsgSubmitProposal")
//                 {
//                     let message_as_u8 = message.value.as_slice();
//                     if let Ok(msg_submit_proposal) = <MsgSubmitProposal as prost::Message>::decode(message_as_u8) {
//                         let proposal_message = Value::MsgSubmitProposal(msg_submit_proposal);
//                         proposal_message.
//                         // Further processing with proposal_message can be done here
//                         // Note: Removed the trailing dot as it was causing a syntax error
//                     }
//                 }
//             }
//         }
//     }

//     Ok(tables)
// }

// Start of Selection
#[substreams::handlers::map]
pub fn ch_out(clock: Clock, block: Block) -> Result<DatabaseChanges, Error> {
    substreams::log::println("ch_out");

    let mut tables: DatabaseChanges = DatabaseChanges::default();

    for tx_as_bytes in block.txs.iter() {
        let tx_as_u8 = tx_as_bytes.as_slice();

        if let Ok(tx) = <Tx as prost::Message>::decode(tx_as_u8) {
            if let Some(body) = tx.body {
                for message in body.messages.iter() {
                    substreams::log::println(message.type_url.as_str());
                    if message.type_url == "/cosmos.gov.v1.MsgSubmitProposal" {
                        if let Ok(msg_submit_proposal) =
                            <MsgSubmitProposal as prost::Message>::decode(message.value.as_slice())
                        {
                            substreams::log::println("MsgSubmitProposal");
                            substreams::log::println(msg_submit_proposal.content.as_ref().unwrap().type_url.as_str());
                            substreams::log::println(msg_submit_proposal.initial_deposit.len().to_string());
                            substreams::log::println(msg_submit_proposal.proposer.as_str());
                            substreams::log::println(msg_submit_proposal.title.as_str());
                            substreams::log::println(msg_submit_proposal.summary.as_str());
                            substreams::log::println(msg_submit_proposal.metadata.as_str());
                            if let Some(content) = msg_submit_proposal.content {
                                substreams::log::println("inside content");

                                if let Ok(msg_software_upgrade) =
                                    <MsgSoftwareUpgrade as prost::Message>::decode(content.value.as_slice())
                                {
                                    substreams::log::println("inside msg_software_upgrade");
                                    let plan_name = msg_software_upgrade
                                        .plan
                                        .as_ref()
                                        .map(|p| p.name.clone())
                                        .unwrap_or_default();
                                    let plan_height =
                                        msg_software_upgrade.plan.as_ref().map(|p| p.height).unwrap_or_default();
                                    let plan_info = msg_software_upgrade
                                        .plan
                                        .as_ref()
                                        .map(|p| p.info.clone())
                                        .unwrap_or_default();

                                    substreams::log::println(format!("plan_name: {}", plan_name));
                                    substreams::log::println(format!("plan_height: {}", plan_height));
                                    substreams::log::println(format!("plan_info: {}", plan_info));
                                } else {
                                    substreams::log::println("couldn't decode software upgrade proposal");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(tables)
}

// pub fn insert_proposal(tables: &mut DatabaseChanges, tx: Transaction) -> DatabaseChanges {
//     let messages = extract_messages(tx.messages);
//     let proposal_message = messages.iter().find(|message| {
//         message.value.is_some() && message.value.unwrap().type_url == "/cosmos.gov.v1beta1.MsgSubmitProposal"
//     });

//     // // TODO: Define table, keys, ordinal, and operation
//     // let row = tables.table_changes.push((table, keys, ordinal, operation));

//     // tables.clone()
// }

// fn extract_messages(messages: Vec<Any>) -> Vec<Message> {
//     return messages
//         .iter()
//         .enumerate()
//         .map(|(u, message)| {
//             let message_as_u8 = &message.value[..];
//             let i = u.try_into().unwrap();

//             match message.type_url.as_str() {
//                 "/cosmos.authz.v1beta1.MsgExec" => {
//                     if let Ok(msg_exec) = <MsgExec as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgExec(msg_exec), i);
//                     }
//                 }
//                 "/cosmos.bank.v1beta1.MsgSend" => {
//                     if let Ok(msg_send) = <MsgSend as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgSend(msg_send), i);
//                     }
//                 }
//                 "/cosmos.bank.v1beta1.MsgMultiSend" => {
//                     if let Ok(msg_multi_send) = <MsgMultiSend as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgMultiSend(msg_multi_send), i);
//                     }
//                 }
//                 "/cosmos.crisis.v1beta1.MsgVerifyInvariant" => {
//                     if let Ok(msg_verify_invariant) = <MsgVerifyInvariant as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgVerifyInvariant(msg_verify_invariant), i);
//                     }
//                 }
//                 "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward" => {
//                     if let Ok(msg_withdraw_delegator_reward) =
//                         <MsgWithdrawDelegatorReward as prost::Message>::decode(message_as_u8)
//                     {
//                         return build_message(Value::MsgWithdrawDelegatorReward(msg_withdraw_delegator_reward), i);
//                     }
//                 }
//                 "/cosmos.distribution.v1beta1.MsgWithdrawValidatorCommission" => {
//                     if let Ok(msg_withdraw_validator_commission) =
//                         <MsgWithdrawValidatorCommission as prost::Message>::decode(message_as_u8)
//                     {
//                         return build_message(
//                             Value::MsgWithdrawValidatorCommission(msg_withdraw_validator_commission),
//                             i,
//                         );
//                     }
//                 }
//                 "/cosmos.distribution.v1beta1.MsgSetWithdrawAddress" => {
//                     if let Ok(msg_set_withdraw_address) =
//                         <MsgSetWithdrawAddress as prost::Message>::decode(message_as_u8)
//                     {
//                         return build_message(Value::MsgSetWithdrawAddress(msg_set_withdraw_address), i);
//                     }
//                 }
//                 "/cosmos.distribution.v1beta1.MsgFundCommunityPool" => {
//                     if let Ok(msg_fund_community_pool) = <MsgFundCommunityPool as prost::Message>::decode(message_as_u8)
//                     {
//                         return build_message(Value::MsgFundCommunityPool(msg_fund_community_pool), i);
//                     }
//                 }
//                 "/cosmos.evidence.v1beta1.MsgSubmitEvidence" => {
//                     if let Ok(msg_submit_evidence) = <MsgSubmitEvidence as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgSubmitEvidence(msg_submit_evidence), i);
//                     }
//                 }
//                 "/cosmos.gov.v1beta1.MsgSubmitProposal" => {
//                     if let Ok(msg_submit_proposal) = <MsgSubmitProposal as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgSubmitProposal(msg_submit_proposal), i);
//                     }
//                 }
//                 "/cosmos.gov.v1beta1.MsgVote" => {
//                     if let Ok(msg_vote) = <MsgVote as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgVote(msg_vote), i);
//                     }
//                 }
//                 "/cosmos.gov.v1beta1.MsgDeposit" => {
//                     if let Ok(msg_deposit) = <MsgDeposit as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgDeposit(msg_deposit), i);
//                     }
//                 }
//                 "/cosmos.slashing.v1beta1.MsgUnjail" => {
//                     if let Ok(msg_unjail) = <MsgUnjail as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgUnjail(msg_unjail), i);
//                     }
//                 }
//                 "/injective.exchange.v1beta1.MsgBatchUpdateOrders" => {
//                     if let Ok(msg) = <MsgBatchUpdateOrders as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgBatchUpdateOrders(msg), i);
//                     }
//                 }
//                 "/injective.wasmx.v1.MsgExecuteContractCompat" => {
//                     if let Ok(msg) = <MsgExecuteContractCompat as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgExecuteContractCompat(msg), i);
//                     }
//                 }
//                 "/injective.auction.v1beta1.MsgBid" => {
//                     if let Ok(msg) = <MsgBid as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgBid(msg), i);
//                     }
//                 }
//                 "/injective.exchange.v1beta.MsgDeposit" => {
//                     if let Ok(msg) = <InjMsgDeposit as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::InjMsgDeposit(msg), i);
//                     }
//                 }
//                 "/injective.peggy.v1.MsgRequestBatch" => {
//                     if let Ok(msg) = <MsgRequestBatch as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgRequestBatch(msg), i);
//                     }
//                 }
//                 "/injective.wasmx.v1.MsgRegisterContract" => {
//                     if let Ok(msg) = <MsgRegisterContract as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgRegisterContract(msg), i);
//                     }
//                 }

//                 "/cosmwasm.wasm.v1.MsgExecuteContract" => {
//                     if let Ok(msg) = <MsgExecuteContract as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgExecuteContract(msg), i);
//                     }
//                 }
//                 "/ibc.core.client.v1.MsgUpdateClient" => {
//                     if let Ok(msg) = <MsgUpdateClient as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgUpdateClient(msg), i);
//                     }
//                 }
//                 "/ibc.core.channel.v1.MsgAcknowledgement" => {
//                     if let Ok(msg) = <MsgAcknowledgement as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgAcknowledgement(msg), i);
//                     }
//                 }
//                 "/injective.oracle.v1beta1.MsgRelayProviderPrices" => {
//                     if let Ok(msg) = <MsgRelayProviderPrices as prost::Message>::decode(message_as_u8) {
//                         return build_message(Value::MsgRelayProviderPrices(msg), i);
//                     }
//                 }
//                 _ => {
//                     log::println(format!("Unsupported message type: {}", message.type_url.as_str()));
//                     return build_message(Value::Other(message.clone()), i);
//                 }
//             }

//             panic!("Could not decode message type {}", message.type_url.as_str());
//         })
//         .collect();
// }

// fn build_message(value: Value, idx: u32) -> Message {
//     return Message {
//         index: idx,
//         value: Some(value),
//     };
// }
