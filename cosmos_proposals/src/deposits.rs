use prost::Message;
use prost_types::Any;
use substreams::store::StoreGet;
use substreams::{pb::substreams::Clock, store::StoreGetString};
use substreams_cosmos::pb::TxResults;
use substreams_entity_change::tables::Tables;

use crate::{
    pb::cosmos::gov::v1beta1::MsgDeposit,
    utils::{add_nanoseconds_to_timestamp, extract_initial_deposit, extract_proposal_status},
};

pub fn create_deposit(
    tables: &mut Tables,
    msg: &Any,
    clock: &Clock,
    tx_result: &TxResults,
    tx_hash: &str,
    gov_params: &StoreGetString,
) {
    if let Ok(msg_deposit) = MsgDeposit::decode(msg.value.as_slice()) {
        let proposal_id = msg_deposit.proposal_id.to_string();
        let depositor = msg_deposit.depositor.as_str();
        let (denom, amount) = extract_initial_deposit(&msg_deposit.amount);

        let proposal_status = extract_proposal_status(tx_result);

        // If this deposit started the deposit period, update the proposal status
        if proposal_status == "VotingPeriod" {
            let timestamp = clock.timestamp.as_ref().expect("timestamp not found");

            let voting_period = gov_params.get_at(0, "voting_period").expect("voting_period not found");
            let voting_end_time = add_nanoseconds_to_timestamp(timestamp, &voting_period);

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
