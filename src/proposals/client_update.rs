use prost_types::Any;
use substreams_entity_change::tables::Tables;

use crate::pb::ibc::core::client::v1::ClientUpdateProposal;

pub fn create_client_update(tables: &mut Tables, content: &Any, proposal_id: &str) {
    if let Ok(msg) = <ClientUpdateProposal as prost::Message>::decode(content.value.as_slice()) {
        tables
            .create_row("ClientUpdate", &proposal_id)
            .set("subject_client_id", msg.subject_client_id.as_str())
            .set("substitute_client_id", msg.substitute_client_id.as_str())
            .set("proposal", proposal_id);
    }
}
