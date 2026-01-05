use crate::client::RpcRequest;
use crate::types::{Commitment, RpcContext};
use serde::{Deserialize, Serialize};

// getVoteAccounts
#[derive(Debug, Clone)]
pub struct RpcGetVoteAccountsRequest {
    pub config: Option<VoteAccountsConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VoteAccountsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vote_pubkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_unstaked_delinquents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent_slot_distance: Option<u64>,
}

impl Serialize for RpcGetVoteAccountsRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = if self.config.is_some() { 1 } else { 0 };
        let mut seq = serializer.serialize_seq(Some(len))?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoteAccount {
    pub vote_pubkey: String,
    pub node_pubkey: String,
    pub activated_stake: u64,
    pub epoch_vote_account: bool,
    pub epoch_credits: u64,
    pub commission: u8,
    pub last_vote: u64,
    pub root_slot: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VoteAccounts {
    pub current: Vec<VoteAccount>,
    pub delinquent: Vec<VoteAccount>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetVoteAccountsResponse {
    pub value: VoteAccounts,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetVoteAccountsRequest {
    fn method_name() -> &'static str {
        "getVoteAccounts"
    }
    
    type Response = RpcGetVoteAccountsResponse;
}


