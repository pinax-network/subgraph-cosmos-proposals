use prost::Message;
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::{
    pb::cosmos::gov::v1beta1::MsgDeposit,
    utils::{extract_initial_deposit, extract_proposal_status},
};

pub fn create_deposit(tables: &mut Tables, msg: &Any, clock: &Clock, tx_result: &TxResults, tx_hash: &str) {
    if let Ok(msg_deposit) = MsgDeposit::decode(msg.value.as_slice()) {
        let proposal_id = msg_deposit.proposal_id.to_string();
        let depositor = msg_deposit.depositor.as_str();
        let (denom, amount) = extract_initial_deposit(&msg_deposit.amount);

        let proposal_status = extract_proposal_status(tx_result);

        // If this deposit started the deposit period, update the proposal status
        if proposal_status == "DepositPeriod" {
            let timestamp = clock.timestamp.as_ref().expect("timestamp not found");
            tables
                .update_row("Proposal", &proposal_id)
                .set("status", proposal_status)
                .set("voting_start_time", timestamp);
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
