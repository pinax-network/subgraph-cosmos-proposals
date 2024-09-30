use prost::Message;
use prost_types::Any;
use substreams::pb::substreams::Clock;
use substreams_entity_change::tables::Tables;

use crate::{pb::cosmos::gov::v1beta1::MsgDeposit, utils::extract_initial_deposit};

pub fn insert_deposit(
    tables: &mut Tables,
    proposal_id: &str,
    amount: &str,
    denom: &str,
    depositor: &str,
    clock: &Clock,
    tx_hash: &str,
) {
    let id = format!("{}-{}", proposal_id, tx_hash);

    tables
        .create_row("Deposit", &id)
        .set("block", &clock.id)
        .set("txHash", tx_hash)
        .set("proposal", proposal_id)
        .set("amount", amount)
        .set("denom", denom)
        .set("depositor", depositor);
}

pub fn insert_deposit_undecoded(tables: &mut Tables, msg: &Any, clock: &Clock, tx_hash: &str) {
    if let Ok(msg_deposit) = MsgDeposit::decode(msg.value.as_slice()) {
        let proposal_id = msg_deposit.proposal_id.to_string();
        let depositor = msg_deposit.depositor.as_str();
        let (denom, amount) = extract_initial_deposit(&msg_deposit.amount);

        insert_deposit(tables, &proposal_id, &amount, &denom, depositor, clock, tx_hash);
    }
}
