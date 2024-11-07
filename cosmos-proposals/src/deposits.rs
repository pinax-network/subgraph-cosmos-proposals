use prost::Message;
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::pb::cosmos::base::v1beta1::Coin;
use crate::utils::GovernanceParamsFlat;
use crate::{
    pb::cosmos::gov::v1beta1::MsgDeposit,
    utils::{add_nanoseconds_to_timestamp, extract_proposal_status},
};

pub fn create_deposit_msg(
    tables: &mut Tables,
    msg: &Any,
    clock: &Clock,
    tx_result: &TxResults,
    tx_hash: &str,
    gov_params: &GovernanceParamsFlat,
) {
    if let Ok(msg_deposit) = MsgDeposit::decode(msg.value.as_slice()) {
        let proposal_id = msg_deposit.proposal_id.to_string();
        let depositor = msg_deposit.depositor.as_str();
        let (denom, amount) = extract_deposit(&msg_deposit.amount);

        let proposal_status = extract_proposal_status(tx_result);

        // If this deposit started the deposit period, update the proposal status
        if proposal_status == "VotingPeriod" {
            let timestamp = clock.timestamp.as_ref().expect("timestamp not found");

            let voting_end_time = add_nanoseconds_to_timestamp(timestamp, &gov_params.voting_period);

            tables
                .update_row("Proposal", &proposal_id)
                .set("status", proposal_status)
                .set("voting_start_time", timestamp)
                .set("voting_end_time", &voting_end_time);
        }

        let id = format!("{}-{}", proposal_id, tx_hash);
        tables
            .create_row("Deposit", &id)
            // @deriveFrom
            .set("block", clock.id.as_str())
            .set("transaction", tx_hash)
            .set("proposal", proposal_id)
            // deposit
            .set("amount", amount)
            .set("denom", denom)
            .set("depositor", depositor);
    }
}

fn extract_deposit(initial_deposit: &[Coin]) -> (&str, &str) {
    initial_deposit
        .get(0)
        .map_or(("", "0"), |deposit| (deposit.denom.as_str(), deposit.amount.as_str()))
}

pub fn create_initial_deposit(
    tables: &mut Tables,
    clock: &Clock,
    tx_result: &TxResults,
    tx_hash: &str,
    proposal_id: &str,
) {
    let (denom, amount) = extract_initial_deposit(&tx_result).unwrap();

    let id = format!("{}-{}", proposal_id, tx_hash);
    tables
        .create_row("Deposit", &id)
        .set("block", clock.id.as_str())
        .set("transaction", tx_hash)
        .set("proposal", proposal_id)
        .set("denom", denom)
        .set("amount", amount);
}

fn extract_initial_deposit(tx_result: &TxResults) -> Option<(String, String)> {
    tx_result
        .events
        .iter()
        .find(|event| event.r#type == "proposal_deposit")
        .and_then(|event| {
            event.attributes.iter().find(|attr| attr.key == "amount").map(|attr| {
                let amount_str = &attr.value;
                // Find the position where numbers end and letters begin
                if let Some(pos) = amount_str.chars().position(|c| c.is_alphabetic()) {
                    let (amount, denom) = amount_str.split_at(pos);
                    (denom.to_string(), amount.to_string())
                } else {
                    ("".to_string(), "0".to_string())
                }
            })
        })
}
