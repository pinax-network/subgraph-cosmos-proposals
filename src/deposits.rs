use prost::Message;
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

use crate::{pb::cosmos::gov::v1beta1::MsgDeposit, utils::extract_initial_deposit};

pub fn create_deposit(tables: &mut Tables, msg: &Any, clock: &Clock, tx_hash: &str) {
    if let Ok(msg_deposit) = MsgDeposit::decode(msg.value.as_slice()) {
        let proposal_id = msg_deposit.proposal_id.to_string();
        let depositor = msg_deposit.depositor.as_str();
        let (denom, amount) = extract_initial_deposit(&msg_deposit.amount);

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
