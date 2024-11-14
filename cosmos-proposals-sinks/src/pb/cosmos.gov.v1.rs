#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitProposal {
    /// content is the proposal's content.
    #[prost(message, repeated, tag = "1")]
    pub messages: Vec<::prost_types::Any>,
    /// initial_deposit is the deposit value that must be paid at proposal submission.
    #[prost(message, repeated, tag = "2")]
    pub initial_deposit: Vec<crate::pb::cosmos::base::v1beta1::Coin>,
    /// proposer is the account address of the proposer.
    #[prost(string, tag = "3")]
    pub proposer: String,
    /// metadata is the metadata of the proposal.
    #[prost(string, tag = "4")]
    pub metadata: ::prost::alloc::string::String,
    /// title is the title of the proposal.
    #[prost(string, tag = "5")]
    pub title: ::prost::alloc::string::String,
    /// summary is the summary of the proposal.
    #[prost(string, tag = "6")]
    pub summary: ::prost::alloc::string::String,
    /// expedited defines if the proposal is expedited
    #[prost(optional, bool, tag = "7")]
    pub expedited: Option<bool>,
    #[prost(optional, enumeration = "ProposalType", tag = "8")]
    pub proposal_type: Option<i32>,
}

/// ProposalType defines the type of proposal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProposalType {
    /// PROPOSAL_TYPE_UNSPECIFIED defines no proposal type, which fallback to PROPOSAL_TYPE_STANDARD.
    Unspecified = 0,
    /// PROPOSAL_TYPE_STANDARD defines the type for a standard proposal.
    Standard = 1,
    /// PROPOSAL_TYPE_MULTIPLE_CHOICE defines the type for a multiple choice proposal.
    MultipleChoice = 2,
    /// PROPOSAL_TYPE_OPTIMISTIC defines the type for an optimistic proposal.
    Optimistic = 3,
    /// PROPOSAL_TYPE_EXPEDITED defines the type for an expedited proposal.
    Expedited = 4,
}
