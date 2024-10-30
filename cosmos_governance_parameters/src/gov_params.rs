use substreams::pb::substreams::Clock;
use substreams::store::StoreGet;
use substreams::store::StoreGetString;

use crate::genesis_params::GovParams;

#[substreams::handlers::store]
pub fn gov_params(
    genesis_params: String,
    clock: Clock,
    proposal_events: ProposalEvents,
    pending_gov_params: StoreGetString,
    gov_params: StoreSetString,
) {
    if clock.number == 1 {
        set_genesis_params(&gov_params, &genesis_params);
    }

    for passed_proposal_id in proposal_events.passed_proposal_ids {
        let gov_param_proposal_str = pending_gov_params.get_at(0, &passed_proposal_id);

        if let Some(gov_param_proposal_str) = gov_param_proposal_str {
            set_new_gov_params(&gov_params, &gov_param_proposal_str);
        }
    }
}

fn set_genesis_params(gov_params: &StoreSetString, genesis_params: &String) {
    let parsed: GovParams = serde_json::from_str(&genesis_params).expect("Failed to parse genesis parameters");

    let min_deposit: String =
        serde_json::to_string(&parsed.deposit_params.min_deposit).expect("Failed to serialize min deposit");
    let max_deposit_period: String = parsed.deposit_params.max_deposit_period;
    let voting_period: String = parsed.voting_params.voting_period.to_string();
    let quorum: String = parsed.tally_params.quorum;
    let threshold: String = parsed.tally_params.threshold;
    let veto_threshold: String = parsed.tally_params.veto_threshold;

    gov_params.set(0, "min_deposit", &min_deposit);
    gov_params.set(0, "max_deposit_period", &max_deposit_period);
    gov_params.set(0, "voting_period", &voting_period);
    gov_params.set(0, "quorum", &quorum);
    gov_params.set(0, "threshold", &threshold);
    gov_params.set(0, "veto_threshold", &veto_threshold);
}

fn set_new_gov_params(gov_params: &StoreSetString, gov_param_proposal_str: &String) {
    let parsed: GovParamsOptional = serde_json::from_str(&gov_param_proposal_str).expect("Failed to parse gov params");

    if let Some(deposit_params) = parsed.deposit_params {
        if deposit_params.min_deposit.len() > 0 {
            let min_deposit_str: String =
                serde_json::to_string(&deposit_params.min_deposit).expect("Failed to serialize min deposit");
            gov_params.set(0, "min_deposit", &min_deposit_str);
        }
        if let Some(max_deposit_period) = deposit_params.max_deposit_period {
            gov_params.set(0, "max_deposit_period", &max_deposit_period);
        }
    }

    if let Some(voting_params) = parsed.voting_params {
        if let Some(voting_period) = voting_params.voting_period {
            gov_params.set(0, "voting_period", &voting_period);
        }
    }

    if let Some(tally_params) = parsed.tally_params {
        if let Some(quorum) = tally_params.quorum {
            gov_params.set(0, "quorum", &quorum);
        }
        if let Some(threshold) = tally_params.threshold {
            gov_params.set(0, "threshold", &threshold);
        }
        if let Some(veto_threshold) = tally_params.veto_threshold {
            gov_params.set(0, "veto_threshold", &veto_threshold);
        }
    }
}
