use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetString;

use crate::pb::cosmos::custom_events::ProposalEvents;
use crate::serde_structs::GovParamsOptional;

#[substreams::handlers::store]
pub fn pending_gov_params(proposal_events: ProposalEvents, store: StoreSetString) {
    for proposal in proposal_events.gov_params_changes {
        let proposal_id = &proposal.proposal_id;

        let gov_param_proposal_str = serde_json::to_string(&proposal).unwrap();
        store.set(0, proposal_id, &gov_param_proposal_str);
    }
}
