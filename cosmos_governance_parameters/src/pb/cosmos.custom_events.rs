use serde::{Deserialize, Serialize}; // Import serde traits

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct ProposalEvents {
    #[prost(message, repeated, tag = "2")]
    pub gov_params_changes: ::prost::alloc::vec::Vec<GovParamsOptional>,
    #[prost(string, repeated, tag = "3")]
    pub passed_proposal_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct DepositParamsOptional {
    #[prost(message, repeated, tag = "1")]
    pub min_deposit: ::prost::alloc::vec::Vec<Deposit>,
    #[prost(string, optional, tag = "2")]
    pub max_deposit_period: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct Deposit {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct VotingParamsOptional {
    #[prost(string, optional, tag = "1")]
    pub voting_period: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct TallyParamsOptional {
    #[prost(string, optional, tag = "1")]
    pub quorum: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub threshold: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub veto_threshold: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]
pub struct GovParamsOptional {
    #[prost(string, tag = "1")]
    pub proposal_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub deposit_params: ::core::option::Option<DepositParamsOptional>,
    #[prost(message, optional, tag = "3")]
    pub voting_params: ::core::option::Option<VotingParamsOptional>,
    #[prost(message, optional, tag = "4")]
    pub tally_params: ::core::option::Option<TallyParamsOptional>,
}
// @@protoc_insertion_point(module)
