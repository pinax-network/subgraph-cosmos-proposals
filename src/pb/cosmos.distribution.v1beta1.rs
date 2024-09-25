#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct CommunityPoolSpendProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<crate::pb::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "5")]
    pub initial_deposit: ::prost::alloc::vec::Vec<crate::pb::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "6")]
    pub proposer: ::prost::alloc::string::String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message, serde::Serialize, serde::Deserialize)]
pub struct MsgCommunityPoolSpend {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<crate::pb::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub initial_deposit: ::prost::alloc::vec::Vec<crate::pb::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub proposer: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub summary: ::prost::alloc::string::String,
}
