use cosmos_proposals_protobuf::pb::cosmos::proposals::v1::Events;
use substreams::store::StoreNew;
use substreams::store::StoreSet;
use substreams::store::StoreSetString;

#[substreams::handlers::store]
pub fn pending_gov_params(events: Events, store: StoreSetString) {
    for event in events.gov_params_changes {
        let proposal_id = &event.proposal_id;
        store.set(0, proposal_id, &event.params);
    }
}