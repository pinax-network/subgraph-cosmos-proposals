use substreams::store::StoreGet;
use substreams::store::StoreGetString;

#[substreams::handlers::store]
pub fn gov_params(proposal_events: ProposalEvents, pending_gov_params: StoreGetString, gov_params: StoreSetString) {
    for passed_proposal_id in proposal_events.passed_proposal_ids {
        let gov_param_proposal_str = pending_gov_params.get_at(0, &passed_proposal_id).unwrap();
        gov_params.set(0, &passed_proposal_id, &gov_param_proposal_str);
    }
}
