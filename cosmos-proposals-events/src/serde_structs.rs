#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProposalEvents {
    pub gov_params_changes: Vec<GovParamsOptional>,
    pub passed_proposal_ids: Vec<String>,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DepositParamsOptional {
    pub min_deposit: Vec<Deposit>,
    pub max_deposit_period: Option<String>,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Deposit {
    pub denom: String,
    pub amount: String,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VotingParamsOptional {
    pub voting_period: Option<String>,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TallyParamsOptional {
    pub quorum: Option<String>,
    pub threshold: Option<String>,
    pub veto_threshold: Option<String>,
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GovParamsOptional {
    pub proposal_id: String,
    pub deposit_params: Option<DepositParamsOptional>,
    pub voting_params: Option<VotingParamsOptional>,
    pub tally_params: Option<TallyParamsOptional>,
}
