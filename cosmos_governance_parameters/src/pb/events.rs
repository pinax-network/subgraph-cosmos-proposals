use prost::Message;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(PartialEq)]
pub struct ProposalEvents {
    #[prost(message, repeated, tag = "1")]
    pub gov_params_changes: Vec<GovParamsOptional>,
    #[prost(string, repeated, tag = "2")]
    pub passed_proposal_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(PartialEq)]
pub struct DepositParamsOptional {
    #[prost(message, repeated, tag = "1")]
    pub min_deposit: Vec<Deposit>,
    #[prost(string, optional, tag = "2")]
    pub max_deposit_period: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(PartialEq)]
pub struct Deposit {
    #[prost(string, tag = "1")]
    pub denom: String,
    #[prost(string, tag = "2")]
    pub amount: String,
}

#[derive(Serialize, Deserialize, Clone, Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(PartialEq)]
pub struct VotingParamsOptional {
    #[prost(string, optional, tag = "1")]
    pub voting_period: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(PartialEq)]
pub struct TallyParamsOptional {
    #[prost(string, optional, tag = "1")]
    pub quorum: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub threshold: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub veto_threshold: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Message)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(PartialEq)]
pub struct GovParamsOptional {
    #[prost(string, tag = "1")]
    pub proposal_id: String,
    #[prost(message, optional, tag = "2")]
    pub deposit_params: Option<DepositParamsOptional>,
    #[prost(message, optional, tag = "3")]
    pub voting_params: Option<VotingParamsOptional>,
    #[prost(message, optional, tag = "4")]
    pub tally_params: Option<TallyParamsOptional>,
}
