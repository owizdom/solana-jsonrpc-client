use crate::client::RpcRequest;
use crate::types::{AccountInfo, Commitment, Encoding, RpcContext};
use serde::{Deserialize, Serialize};

// getProgramAccounts
#[derive(Debug, Clone)]
pub struct RpcGetProgramAccountsRequest {
    pub program_id: String,
    pub config: Option<ProgramAccountsConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProgramAccountsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ProgramAccountFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSliceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_context: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ProgramAccountFilter {
    Memcmp {
        #[serde(rename = "memcmp")]
        memcmp: MemcmpFilter,
    },
    DataSize {
        #[serde(rename = "dataSize")]
        data_size: usize,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct MemcmpFilter {
    pub offset: usize,
    pub bytes: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DataSliceConfig {
    pub offset: usize,
    pub length: usize,
}

impl Serialize for RpcGetProgramAccountsRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.program_id)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProgramAccount {
    pub account: AccountInfo,
    pub pubkey: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetProgramAccountsResponse {
    pub value: Vec<ProgramAccount>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetProgramAccountsRequest {
    fn method_name() -> &'static str {
        "getProgramAccounts"
    }
    
    type Response = RpcGetProgramAccountsResponse;
}

