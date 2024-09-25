#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitProposal {
    /// content is the proposal's content.
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<::prost_types::Any>,
    /// initial_deposit is the deposit value that must be paid at proposal submission.
    #[prost(message, repeated, tag = "2")]
    pub initial_deposit: ::prost::alloc::vec::Vec<crate::pb::cosmos::base::v1beta1::Coin>,
    /// proposer is the account address of the proposer.
    #[prost(string, tag = "3")]
    pub proposer: ::prost::alloc::string::String,
    /// metadata is the metadata of the proposal.
    /// metadata is the metadata of the proposal.
    #[prost(string, optional, tag = "4")]
    pub metadata: ::core::option::Option<::prost::alloc::string::String>,
    /// title is the title of the proposal.
    #[prost(string, optional, tag = "5")]
    pub title: ::core::option::Option<::prost::alloc::string::String>,
    /// summary is the summary of the proposal.
    #[prost(string, optional, tag = "6")]
    pub summary: ::core::option::Option<::prost::alloc::string::String>,
}
