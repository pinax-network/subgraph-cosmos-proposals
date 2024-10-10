use crate::pb::cosmos::params::v1beta1::ParameterChangeProposal;
use prost_types::Any;
use substreams_entity_change::tables::Tables;

pub fn create_parameter_change_proposal(tables: &mut Tables, content: &Any, proposal_id: &str) {
    if let Ok(msg) = <ParameterChangeProposal as prost::Message>::decode(content.value.as_slice()) {
        msg.changes.iter().for_each(|change| {
            tables
                .create_row("ParameterChange", &proposal_id)
                .set("subspace", change.subspace.as_str())
                .set("key", change.key.as_str())
                .set("value", change.value.as_str())
                .set("proposal", proposal_id);
        });
    }
}
