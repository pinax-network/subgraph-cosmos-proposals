use cosmos_proposals_protobuf::pb::cosmos::proposals::v1::Events;
use cosmos_proposals_protobuf::pb::cosmos::proposals::v1::GovParamsOptional;
use cosmos_proposals_protobuf::pb::cosmos::proposals::v1::NewProposalWithType;
use substreams::pb::substreams::Clock;
use substreams::store::StoreGet;
use substreams::store::StoreGetString;
use substreams::store::StoreNew;

use substreams::store::StoreSet;
use substreams::store::StoreSetString;

#[substreams::handlers::store]
pub fn gov_params(
    genesis_params: String,
    clock: Clock,
    events: Events,
    pending_gov_params: StoreGetString,
    gov_params: StoreSetString,
) {
    if clock.number == 1 {
        if genesis_params.is_empty() {
            return;
        }
        set_new_gov_params(&gov_params, &genesis_params);
        gov_params.set(0, "block_id_last_updated", &clock.id);
    }

    for passed_proposal_id in events.passed_proposal_ids {
        let gov_param_proposal_str = pending_gov_params.get_at(0, &passed_proposal_id);

        if let Some(gov_param_proposal_str) = gov_param_proposal_str {
            gov_params.set(0, "block_id_last_updated", &clock.id);
            set_new_gov_params(&gov_params, &gov_param_proposal_str);
        }
    }

    // Set proposal types for new proposal
    // e.g Standard, MultipleChoice, Optimistic, Expedited
    for new_proposal in events.new_proposals_with_types {
        set_proposal_type(&gov_params, &new_proposal);
    }
}

fn set_new_gov_params(gov_params: &StoreSetString, gov_param_proposal_str: &String) {
    // There are some discrepancies in the parameter names between chains
    // Cosmos & Injective use "veto_threshold" in their genesis files, but then use "veto" in documentation
    // Osmosis uses "min_expedited_deposit" instead of "expedited_min_deposit"
    // https://docs.injective.network/developers/modules/core/gov#parameters
    // https://docs.osmosis.zone/osmosis-core/modules/gov
    let formatted_str = gov_param_proposal_str
        .replace("\"veto\"", "\"veto_threshold\"")
        .replace("\"min_expedited_deposit\"", "\"expedited_min_deposit\"");

    let parsed: GovParamsOptional = serde_json::from_str(&formatted_str).expect("Failed to parse gov params");

    if let Some(deposit_params) = parsed.deposit_params {
        if deposit_params.min_deposit.len() > 0 {
            let min_deposit_str: String =
                serde_json::to_string(&deposit_params.min_deposit).expect("Failed to serialize min deposit");
            gov_params.set(0, "min_deposit", &min_deposit_str);
        }
        if deposit_params.expedited_min_deposit.len() > 0 {
            let expedited_min_deposit_str: String = serde_json::to_string(&deposit_params.expedited_min_deposit)
                .expect("Failed to serialize expedited min deposit");
            gov_params.set(0, "expedited_min_deposit", &expedited_min_deposit_str);
        }
        if let Some(max_deposit_period) = deposit_params.max_deposit_period {
            gov_params.set(0, "max_deposit_period", &max_deposit_period);
        }
    }

    if let Some(voting_params) = parsed.voting_params {
        if let Some(voting_period) = voting_params.voting_period {
            gov_params.set(0, "voting_period", &voting_period);
        }
        if let Some(expedited_voting_period) = voting_params.expedited_voting_period {
            gov_params.set(0, "expedited_voting_period", &expedited_voting_period);
        }
    }

    if let Some(tally_params) = parsed.tally_params {
        if let Some(quorum) = tally_params.quorum {
            gov_params.set(0, "quorum", &quorum);
        }
        if let Some(expedited_quorum) = tally_params.expedited_quorum {
            gov_params.set(0, "expedited_quorum", &expedited_quorum);
        }
        if let Some(threshold) = tally_params.threshold {
            gov_params.set(0, "threshold", &threshold);
        }
        if let Some(expedited_threshold) = tally_params.expedited_threshold {
            gov_params.set(0, "expedited_threshold", &expedited_threshold);
        }
        if let Some(veto_threshold) = tally_params.veto_threshold {
            gov_params.set(0, "veto_threshold", &veto_threshold);
        }
    }
}

fn set_proposal_type(gov_params: &StoreSetString, new_proposal: &NewProposalWithType) {
    gov_params.set(
        0,
        format!("proposal_id_type:{}", new_proposal.proposal_id),
        &new_proposal.proposal_type,
    );
}
