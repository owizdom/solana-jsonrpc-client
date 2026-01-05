use crate::client::RpcRequest;
use crate::types::{Commitment, Encoding, RpcContext};
use serde::{Deserialize, Serialize};

// getTokenAccountBalance
#[derive(Debug, Clone)]
pub struct RpcGetTokenAccountBalanceRequest {
    pub account: String,
    pub commitment: Option<Commitment>,
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetTokenAccountBalanceRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = 1 + if self.commitment.is_some() || self.min_context_slot.is_some() { 1 } else { 0 };
        let mut seq = serializer.serialize_seq(Some(len))?;
        seq.serialize_element(&self.account)?;
        if self.commitment.is_some() || self.min_context_slot.is_some() {
            #[derive(Serialize)]
            struct Config {
                #[serde(skip_serializing_if = "Option::is_none")]
                commitment: Option<Commitment>,
                #[serde(skip_serializing_if = "Option::is_none")]
                min_context_slot: Option<u64>,
            }
            seq.serialize_element(&Config {
                commitment: self.commitment,
                min_context_slot: self.min_context_slot,
            })?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenAmount {
    pub ui_amount: Option<f64>,
    pub decimals: u8,
    pub amount: String,
    pub ui_amount_string: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetTokenAccountBalanceResponse {
    pub value: TokenAmount,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetTokenAccountBalanceRequest {
    fn method_name() -> &'static str {
        "getTokenAccountBalance"
    }
    
    type Response = RpcGetTokenAccountBalanceResponse;
}

// getTokenAccountsByDelegate
#[derive(Debug, Clone)]
pub struct RpcGetTokenAccountsByDelegateRequest {
    pub delegate: String,
    pub config: TokenAccountsConfig,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenAccountsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetTokenAccountsByDelegateRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.delegate)?;
        seq.serialize_element(&self.config)?;
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenAccount {
    pub account: serde_json::Value,
    pub pubkey: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetTokenAccountsByDelegateResponse {
    pub value: Vec<TokenAccount>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetTokenAccountsByDelegateRequest {
    fn method_name() -> &'static str {
        "getTokenAccountsByDelegate"
    }
    
    type Response = RpcGetTokenAccountsByDelegateResponse;
}

// getTokenAccountsByOwner
#[derive(Debug, Clone)]
pub struct RpcGetTokenAccountsByOwnerRequest {
    pub owner: String,
    pub config: TokenAccountsConfig,
}

impl Serialize for RpcGetTokenAccountsByOwnerRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.owner)?;
        seq.serialize_element(&self.config)?;
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetTokenAccountsByOwnerResponse {
    pub value: Vec<TokenAccount>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetTokenAccountsByOwnerRequest {
    fn method_name() -> &'static str {
        "getTokenAccountsByOwner"
    }
    
    type Response = RpcGetTokenAccountsByOwnerResponse;
}

// getTokenLargestAccounts
#[derive(Debug, Clone)]
pub struct RpcGetTokenLargestAccountsRequest {
    pub mint: String,
    pub commitment: Option<Commitment>,
}

impl Serialize for RpcGetTokenLargestAccountsRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.commitment.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.mint)?;
        if let Some(ref commitment) = self.commitment {
            seq.serialize_element(commitment)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenAccountBalance {
    pub address: String,
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetTokenLargestAccountsResponse {
    pub value: Vec<TokenAccountBalance>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetTokenLargestAccountsRequest {
    fn method_name() -> &'static str {
        "getTokenLargestAccounts"
    }
    
    type Response = RpcGetTokenLargestAccountsResponse;
}

// getTokenSupply
#[derive(Debug, Clone)]
pub struct RpcGetTokenSupplyRequest {
    pub mint: String,
    pub commitment: Option<Commitment>,
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetTokenSupplyRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = 1 + if self.commitment.is_some() || self.min_context_slot.is_some() { 1 } else { 0 };
        let mut seq = serializer.serialize_seq(Some(len))?;
        seq.serialize_element(&self.mint)?;
        if self.commitment.is_some() || self.min_context_slot.is_some() {
            #[derive(Serialize)]
            struct Config {
                #[serde(skip_serializing_if = "Option::is_none")]
                commitment: Option<Commitment>,
                #[serde(skip_serializing_if = "Option::is_none")]
                min_context_slot: Option<u64>,
            }
            seq.serialize_element(&Config {
                commitment: self.commitment,
                min_context_slot: self.min_context_slot,
            })?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TokenSupply {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: Option<f64>,
    pub ui_amount_string: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetTokenSupplyResponse {
    pub value: TokenSupply,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetTokenSupplyRequest {
    fn method_name() -> &'static str {
        "getTokenSupply"
    }
    
    type Response = RpcGetTokenSupplyResponse;
}


