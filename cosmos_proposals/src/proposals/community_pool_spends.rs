use prost_types::Any;
use substreams_entity_change::tables::Tables;

use crate::pb::cosmos::distribution::v1beta1::MsgCommunityPoolSpend;

pub fn create_community_pool_spend(tables: &mut Tables, content: &Any, proposal_id: &str) {
    if let Ok(msg) = <MsgCommunityPoolSpend as prost::Message>::decode(content.value.as_slice()) {
        let coin = msg.amount.get(0).expect("missing amount");
        tables
            .create_row("CommunityPoolSpend", &proposal_id)
            .set("authority", msg.authority.as_str())
            .set("recipient", msg.recipient.as_str())
            .set_bigint("amount", &coin.amount.to_string())
            .set("denom", coin.denom.as_str())
            .set("proposal", proposal_id);
    }
}
