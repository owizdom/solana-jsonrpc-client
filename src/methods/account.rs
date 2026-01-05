use crate::client::RpcRequest;
use crate::types::{AccountInfo, Commitment, Encoding, RpcContext};
use serde::{Deserialize, Serialize};

// getAccountInfo
#[derive(Debug, Clone)]
pub struct RpcGetAccountInfoRequest {
    pub pubkey: String,
    pub config: Option<AccountInfoConfig>,
}

impl Serialize for RpcGetAccountInfoRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.pubkey)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AccountInfoConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_slice: Option<DataSliceConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DataSliceConfig {
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetAccountInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<AccountInfo>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetAccountInfoRequest {
    fn method_name() -> &'static str {
        "getAccountInfo"
    }
    
    type Response = RpcGetAccountInfoResponse;
}

// getBalance
#[derive(Debug, Clone)]
pub struct RpcGetBalanceRequest {
    pub pubkey: String,
    pub config: Option<BalanceConfig>,
}

impl Serialize for RpcGetBalanceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.pubkey)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BalanceConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetBalanceResponse {
    pub value: u64,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetBalanceRequest {
    fn method_name() -> &'static str {
        "getBalance"
    }
    
    type Response = RpcGetBalanceResponse;
}

// getMultipleAccounts
#[derive(Debug, Clone)]
pub struct RpcGetMultipleAccountsRequest {
    pub pubkeys: Vec<String>,
    pub config: Option<AccountInfoConfig>,
}

impl Serialize for RpcGetMultipleAccountsRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.pubkeys)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetMultipleAccountsResponse {
    pub value: Vec<Option<AccountInfo>>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetMultipleAccountsRequest {
    fn method_name() -> &'static str {
        "getMultipleAccounts"
    }
    
    type Response = RpcGetMultipleAccountsResponse;
}

// getLargestAccounts
#[derive(Debug, Clone)]
pub struct RpcGetLargestAccountsRequest {
    pub config: Option<LargestAccountsConfig>,
}

impl Serialize for RpcGetLargestAccountsRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 1 } else { 0 }))?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LargestAccountsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LargestAccountsFilter>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum LargestAccountsFilter {
    Circulating,
    NonCirculating,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AccountBalance {
    pub address: String,
    pub lamports: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetLargestAccountsResponse {
    pub value: Vec<AccountBalance>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetLargestAccountsRequest {
    fn method_name() -> &'static str {
        "getLargestAccounts"
    }
    
    type Response = RpcGetLargestAccountsResponse;
}

// getMinimumBalanceForRentExemption
#[derive(Debug, Clone)]
pub struct RpcGetMinimumBalanceForRentExemptionRequest {
    pub data_len: usize,
    pub commitment: Option<Commitment>,
}

impl Serialize for RpcGetMinimumBalanceForRentExemptionRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.commitment.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.data_len)?;
        if let Some(ref commitment) = self.commitment {
            seq.serialize_element(commitment)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetMinimumBalanceForRentExemptionResponse {
    pub value: u64,
}

impl RpcRequest for RpcGetMinimumBalanceForRentExemptionRequest {
    fn method_name() -> &'static str {
        "getMinimumBalanceForRentExemption"
    }
    
    type Response = RpcGetMinimumBalanceForRentExemptionResponse;
}

