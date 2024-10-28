use crate::deposits::create_deposit;
use crate::genesis_params::GovParams;
use crate::gov_params_store::{
    self, DepositParamsOptional, GovParamsOptional, TallyParamsOptional, VotingParamsOptional,
};
use crate::pb::cosmos::{
    authz::v1beta1::MsgExec,
    gov::v1::MsgSubmitProposal as MsgSubmitProposalV1,
    gov::v1beta1::{MsgSubmitProposal as MsgSubmitProposalV1Beta1, TextProposal},
};
use crate::sink::GovParamsStore;
use crate::utils::{add_nanoseconds_to_timestamp, extract_authority, extract_proposal_id, extract_proposal_status};
use crate::votes::create_vote;
use prost::Message;
use prost_types::Any;
use substreams::{pb::substreams::Clock, Hex};
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::{Row, Tables};

use super::{
    client_update::create_client_update, community_pool_spends::create_community_pool_spend,
    parameter_changes::create_parameter_change_proposal, software_upgrades::create_software_upgrade,
};

pub fn handle_proposals(
    tables: &mut Tables,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    gov_params: &GovParamsStore,
) {
    let proposal_id = extract_proposal_id(tx_result, clock, tx_hash);
    let status = extract_proposal_status(tx_result);

    match message.type_url.as_str() {
        "/cosmos.gov.v1.MsgSubmitProposal" => handle_v1_proposal(
            tables,
            clock,
            message,
            tx_result,
            tx_hash,
            &proposal_id,
            &status,
            gov_params,
        ),
        "/cosmos.gov.v1beta1.MsgSubmitProposal" => handle_v1beta1_proposal(
            tables,
            clock,
            message,
            tx_result,
            tx_hash,
            &proposal_id,
            &status,
            gov_params,
        ),
        "/cosmos.authz.v1beta1.MsgExec" => handle_exec_proposal(
            tables,
            clock,
            message,
            tx_result,
            tx_hash,
            &proposal_id,
            &status,
            gov_params,
        ),
        "/cosmos.gov.v1beta1.MsgVote" => create_vote(tables, message, tx_result, clock, tx_hash),
        "/cosmos.gov.v1beta1.MsgDeposit" => create_deposit(tables, message, clock, tx_result, tx_hash, gov_params),
        _ => {}
    }
}

fn handle_v1_proposal(
    tables: &mut Tables,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    proposal_id: &str,
    status: &str,
    gov_params: &GovParamsStore,
) {
    if let Ok(msg) = MsgSubmitProposalV1::decode(message.value.as_slice()) {
        let row = tables.create_row("Proposal", proposal_id);
        set_proposal_entity(row, clock, message, tx_result, tx_hash, status, gov_params);
        set_proposal_v1(row, &msg);
        set_proposal_messages(tables, &msg, proposal_id);
    }
}

fn handle_v1beta1_proposal(
    tables: &mut Tables,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    proposal_id: &str,
    status: &str,
    gov_params: &GovParamsStore,
) {
    if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(message.value.as_slice()) {
        let row = tables.create_row("Proposal", proposal_id);
        set_proposal_entity(row, clock, message, tx_result, tx_hash, status, gov_params);
        set_proposal_v1beta1(row, &msg);
        set_proposal_messages(tables, &msg, proposal_id);

        if let Some(first_message) = msg.messages.first() {
            handle_specific_proposal(tables, first_message, proposal_id);
        }
    }
}

fn handle_exec_proposal(
    tables: &mut Tables,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    proposal_id: &str,
    status: &str,
    gov_params: &GovParamsStore,
) {
    if let Ok(msg_exec) = MsgExec::decode(message.value.as_slice()) {
        for msg in msg_exec.msgs {
            let row = tables.create_row("Proposal", proposal_id);
            set_proposal_entity(row, clock, message, tx_result, tx_hash, status, gov_params);

            if let Ok(msg) = MsgSubmitProposalV1::decode(msg.value.as_slice()) {
                set_proposal_v1(row, &msg);
                set_proposal_messages(tables, &msg, proposal_id);
            } else if let Ok(msg) = MsgSubmitProposalV1Beta1::decode(msg.value.as_slice()) {
                set_proposal_v1beta1(row, &msg);
                set_proposal_messages(tables, &msg, proposal_id);
            }
        }
    }
}

fn handle_specific_proposal(tables: &mut Tables, message: &Any, proposal_id: &str) {
    match message.type_url.as_str() {
        "/cosmos.upgrade.v1beta1.SoftwareUpgradeProposal" => create_software_upgrade(tables, message, proposal_id),
        "/cosmos.params.v1beta1.ParameterChangeProposal" => {
            create_parameter_change_proposal(tables, message, proposal_id)
        }
        "/cosmos.distribution.v1beta1.CommunityPoolSpendProposal" => {
            create_community_pool_spend(tables, message, proposal_id)
        }
        "/ibc.core.client.v1.ClientUpdateProposal" => create_client_update(tables, message, proposal_id),
        _ => {}
    }
}

fn set_proposal_v1beta1(row: &mut Row, msg: &MsgSubmitProposalV1Beta1) {
    if let Some(first_message) = msg.messages.first() {
        let proposer = msg.proposer.as_str();
        let (title, summary) = decode_text_proposal(first_message);
        set_proposal_metadata(row, proposer, &title, &summary, "", "Standard");
    }
}

fn set_proposal_v1(row: &mut Row, msg: &MsgSubmitProposalV1) {
    let proposer = msg.proposer.as_str();
    let title = msg.title.as_str();
    let summary = msg.summary.as_str();
    let metadata = msg.metadata.as_str();
    let type_str = determine_proposal_type(msg);
    set_proposal_metadata(row, proposer, title, summary, metadata, &type_str);
}

fn determine_proposal_type(msg: &MsgSubmitProposalV1) -> String {
    match msg.proposal_type.unwrap_or(-1) {
        -1 => {
            if msg.expedited.unwrap_or(false) {
                "Expedited".to_string()
            } else {
                "Standard".to_string()
            }
        }
        proposal_type => proposal_type_to_string(proposal_type),
    }
}

fn set_proposal_messages<T: HasMessages>(tables: &mut Tables, msg: &T, proposal_id: &str) {
    for (i, message) in msg.get_messages().iter().enumerate() {
        let id = format!("{}-{}", proposal_id, i);
        tables
            .create_row("ProposalMessage", &id)
            .set("message_index", i as u8)
            .set("type", message.type_url.as_str())
            .set("raw_data", Hex::encode(&message.value))
            .set("proposal", proposal_id);
    }
}

trait HasMessages {
    fn get_messages(&self) -> &Vec<Any>;
}

impl HasMessages for MsgSubmitProposalV1 {
    fn get_messages(&self) -> &Vec<Any> {
        &self.messages
    }
}

impl HasMessages for MsgSubmitProposalV1Beta1 {
    fn get_messages(&self) -> &Vec<Any> {
        &self.messages
    }
}

pub fn set_proposal_metadata(
    row: &mut Row,
    proposer: &str,
    title: &str,
    summary: &str,
    metadata: &str,
    proposal_type: &str,
) {
    row.set("proposer", proposer)
        .set("title", title)
        .set("summary", summary)
        .set("metadata", metadata)
        .set("proposal_type", proposal_type);
}

pub fn set_proposal_entity(
    row: &mut Row,
    clock: &Clock,
    message: &Any,
    tx_result: &TxResults,
    tx_hash: &str,
    status: &str,
    gov_params: &GovParamsStore,
) {
    let authority = extract_authority(tx_result);
    if message.type_url.is_empty() {
        panic!("Empty type_url in proposal");
    }

    let gov_params = determine_gov_params(gov_params);

    let submit_time = clock.timestamp.as_ref().expect("missing timestamp");

    let tally_params = gov_params.tally_params.as_ref().unwrap();
    let deposit_params = gov_params.deposit_params.as_ref().unwrap();
    let voting_params = gov_params.voting_params.as_ref().unwrap();

    let quorum = tally_params.quorum.as_ref().unwrap();
    let threshold = tally_params.threshold.as_ref().unwrap();
    let veto_threshold = tally_params.veto_threshold.as_ref().unwrap();

    let min_deposit = deposit_params.min_deposit.as_ref().unwrap();
    let mut min_deposit_str: Vec<String> = vec![];
    for deposit in min_deposit {
        min_deposit_str.push(format! {"{} {}", deposit.amount, deposit.denom});
    }

    let max_deposit_period = deposit_params.max_deposit_period.as_ref().unwrap();
    let voting_period = voting_params.voting_period.as_ref().unwrap();

    let deposit_end_time = add_nanoseconds_to_timestamp(submit_time, max_deposit_period);

    row.set("transaction", tx_hash)
        .set("block", &clock.id)
        .set("authority", authority)
        .set("type", &message.type_url)
        .set("status", status)
        .set("submit_time", submit_time)
        .set("deposit_end_time", deposit_end_time)
        .set("quorum", quorum)
        .set("threshold", threshold)
        .set("veto_threshold", veto_threshold)
        .set("min_deposit", min_deposit_str)
        .set_bigint("max_deposit_period", &max_deposit_period)
        .set_bigint("voting_period", &voting_period);

    if status == "VotingPeriod" {
        row.set("voting_start_time", submit_time);
    }
}

pub fn decode_text_proposal(content: &Any) -> (String, String) {
    TextProposal::decode(content.value.as_slice())
        .map(|decoded| (decoded.title, decoded.description))
        .unwrap_or_default()
}

fn proposal_type_to_string(proposal_type: i32) -> String {
    match proposal_type {
        0 | 1 => "Standard",
        2 => "MultipleChoice",
        3 => "Optimistic",
        4 => "Expedited",
        _ => "Unknown",
    }
    .to_string()
}

pub fn determine_gov_params(gov_params_store: &GovParamsStore) -> GovParamsOptional {
    let mut proposals: Vec<GovParamsOptional> = vec![];

    for proposal_str in gov_params_store.param_proposals.iter() {
        let proposal: GovParamsOptional = serde_json::from_str(proposal_str).unwrap();
        if gov_params_store
            .passed_proposal_ids
            .contains(&proposal.proposal_id.as_ref().unwrap())
        {
            proposals.push(proposal);
        }
    }

    // Reverse the order of the proposals so the most recent ones are first
    proposals.reverse();

    let mut gov_params = GovParamsOptional {
        proposal_id: None,
        deposit_params: None,
        voting_params: None,
        tally_params: None,
    };

    // First try to populate from proposals
    for proposal in proposals {
        if let Some(deposit_params) = proposal.deposit_params {
            if let Some(existing_params) = &mut gov_params.deposit_params {
                // Only update fields that aren't already set
                if existing_params.min_deposit.is_none() {
                    existing_params.min_deposit = deposit_params.min_deposit;
                }
                if existing_params.max_deposit_period.is_none() {
                    existing_params.max_deposit_period = deposit_params.max_deposit_period;
                }
            } else {
                // No existing params, set them all
                gov_params.deposit_params = Some(DepositParamsOptional {
                    min_deposit: deposit_params.min_deposit,
                    max_deposit_period: deposit_params.max_deposit_period,
                });
            }
        }

        if let Some(voting_params) = proposal.voting_params {
            if let Some(existing_params) = &mut gov_params.voting_params {
                // Only update fields that aren't already set
                if existing_params.voting_period.is_none() {
                    existing_params.voting_period = voting_params.voting_period;
                }
            } else {
                // No existing params, set them all
                gov_params.voting_params = Some(VotingParamsOptional {
                    voting_period: voting_params.voting_period,
                });
            }
        }

        if let Some(tally_params) = proposal.tally_params {
            if let Some(existing_params) = &mut gov_params.tally_params {
                // Only update fields that aren't already set
                if existing_params.quorum.is_none() {
                    existing_params.quorum = tally_params.quorum;
                }
                if existing_params.threshold.is_none() {
                    existing_params.threshold = tally_params.threshold;
                }
                if existing_params.veto_threshold.is_none() {
                    existing_params.veto_threshold = tally_params.veto_threshold;
                }
            } else {
                // No existing params, set them all
                gov_params.tally_params = Some(TallyParamsOptional {
                    quorum: tally_params.quorum,
                    threshold: tally_params.threshold,
                    veto_threshold: tally_params.veto_threshold,
                });
            }
        }

        // Break early if we have all params fully populated
        let all_deposit_params_set = gov_params
            .deposit_params
            .as_ref()
            .map_or(false, |p| p.min_deposit.is_some() && p.max_deposit_period.is_some());
        let all_voting_params_set = gov_params
            .voting_params
            .as_ref()
            .map_or(false, |p| p.voting_period.is_some());
        let all_tally_params_set = gov_params.tally_params.as_ref().map_or(false, |p| {
            p.quorum.is_some() && p.threshold.is_some() && p.veto_threshold.is_some()
        });

        if all_deposit_params_set && all_voting_params_set && all_tally_params_set {
            break;
        }
    }

    // Fill in any missing params from genesis
    let genesis_params: GovParams = serde_json::from_str(&gov_params_store.genesis_params).unwrap();

    if gov_params.deposit_params.is_none() {
        gov_params.deposit_params = Some(DepositParamsOptional {
            min_deposit: Some(
                genesis_params
                    .deposit_params
                    .min_deposit
                    .into_iter()
                    .map(|d| gov_params_store::Deposit {
                        denom: d.denom,
                        amount: d.amount,
                    })
                    .collect(),
            ),
            max_deposit_period: Some(genesis_params.deposit_params.max_deposit_period),
        });
    } else if let Some(deposit_params) = &mut gov_params.deposit_params {
        if deposit_params.min_deposit.is_none() {
            deposit_params.min_deposit = Some(
                genesis_params
                    .deposit_params
                    .min_deposit
                    .into_iter()
                    .map(|d| gov_params_store::Deposit {
                        denom: d.denom,
                        amount: d.amount,
                    })
                    .collect(),
            );
        }
        if deposit_params.max_deposit_period.is_none() {
            deposit_params.max_deposit_period = Some(genesis_params.deposit_params.max_deposit_period);
        }
    }

    if gov_params.voting_params.is_none() {
        gov_params.voting_params = Some(VotingParamsOptional {
            voting_period: Some(genesis_params.voting_params.voting_period),
        });
    } else if let Some(voting_params) = &mut gov_params.voting_params {
        if voting_params.voting_period.is_none() {
            voting_params.voting_period = Some(genesis_params.voting_params.voting_period);
        }
    }

    if gov_params.tally_params.is_none() {
        gov_params.tally_params = Some(TallyParamsOptional {
            quorum: Some(genesis_params.tally_params.quorum),
            threshold: Some(genesis_params.tally_params.threshold),
            veto_threshold: Some(genesis_params.tally_params.veto_threshold),
        });
    } else if let Some(tally_params) = &mut gov_params.tally_params {
        if tally_params.quorum.is_none() {
            tally_params.quorum = Some(genesis_params.tally_params.quorum);
        }
        if tally_params.threshold.is_none() {
            tally_params.threshold = Some(genesis_params.tally_params.threshold);
        }
        if tally_params.veto_threshold.is_none() {
            tally_params.veto_threshold = Some(genesis_params.tally_params.veto_threshold);
        }
    }

    gov_params
}
