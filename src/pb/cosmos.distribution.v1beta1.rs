use prost::Message;
use serde::{Deserialize, Serialize};

// Main Struct
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct CommunityPoolSpendProposal {
    #[prost(string, tag = "1")]
    pub title: String,
    #[prost(string, tag = "2")]
    pub description: String,
    #[prost(string, tag = "3")]
    pub recipient: String,
    #[prost(message, repeated, tag = "7")]
    pub amount: Vec<crate::pb::cosmos::base::v1beta1::Coin>,
}

// Example of another Message Struct
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
pub struct MsgCommunityPoolSpend {
    #[prost(string, tag = "1")]
    pub authority: String,
    #[prost(string, tag = "2")]
    pub recipient: String,
    #[prost(message, repeated, tag = "3")]
    pub amount: Vec<crate::pb::cosmos::base::v1beta1::Coin>,
}
