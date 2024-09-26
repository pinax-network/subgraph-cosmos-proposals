#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientUpdateProposal {
    /// title is the title of the proposal.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,

    /// description is the description of the proposal.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,

    /// subject_client_id is the ID of the client to be updated.
    #[prost(string, tag = "3")]
    pub subject_client_id: ::prost::alloc::string::String,

    /// substitute_client_id is the ID of the substitute client.
    #[prost(string, tag = "4")]
    pub substitute_client_id: ::prost::alloc::string::String,
}
