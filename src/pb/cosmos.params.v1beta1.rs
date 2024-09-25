/// ParameterChangeProposal defines a governance proposal for parameter changes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ParameterChangeProposal {
    /// title of the proposal.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,

    /// description of the proposal.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,

    /// changes defines the parameter changes.
    #[prost(message, repeated, tag = "3")]
    pub changes: ::prost::alloc::vec::Vec<ParamChange>,

    /// initial_deposit is the deposit value that must be paid at proposal submission.
    #[prost(message, repeated, tag = "4")]
    pub initial_deposit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,

    /// proposer is the account address of the proposer.
    #[prost(string, tag = "5")]
    pub proposer: ::prost::alloc::string::String,
}

/// ParamChange defines a single parameter change.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct ParamChange {
    /// subspace defines the module to which the parameter belongs.
    #[prost(string, tag = "1")]
    pub subspace: ::prost::alloc::string::String,

    /// key defines the key of the parameter.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,

    /// value defines the value of the parameter as a string.
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
}
