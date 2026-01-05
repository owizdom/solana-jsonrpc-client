use crate::client::RpcRequest;
use crate::types::{Commitment, RpcContext, Transaction, TransactionEncoding};
use serde::{Deserialize, Serialize};

// getTransaction
#[derive(Debug, Clone)]
pub struct RpcGetTransactionRequest {
    pub signature: String,
    pub config: Option<TransactionConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransactionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<TransactionEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
}

impl Serialize for RpcGetTransactionRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.signature)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetTransactionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Transaction>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetTransactionRequest {
    fn method_name() -> &'static str {
        "getTransaction"
    }
    
    type Response = RpcGetTransactionResponse;
}

// getTransactionCount
#[derive(Debug, Clone)]
pub struct RpcGetTransactionCountRequest {
    pub commitment: Option<Commitment>,
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetTransactionCountRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = if self.commitment.is_some() || self.min_context_slot.is_some() { 1 } else { 0 };
        let mut seq = serializer.serialize_seq(Some(len))?;
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
pub struct RpcGetTransactionCountResponse {
    pub value: u64,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetTransactionCountRequest {
    fn method_name() -> &'static str {
        "getTransactionCount"
    }
    
    type Response = RpcGetTransactionCountResponse;
}

// sendTransaction
#[derive(Debug, Clone)]
pub struct RpcSendTransactionRequest {
    pub transaction: String, // base64 encoded transaction
    pub config: Option<SendTransactionConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SendTransactionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_preflight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preflight_commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<TransactionEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcSendTransactionRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.transaction)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcSendTransactionResponse {
    pub value: String, // transaction signature
}

impl RpcRequest for RpcSendTransactionRequest {
    fn method_name() -> &'static str {
        "sendTransaction"
    }
    
    type Response = RpcSendTransactionResponse;
}

// simulateTransaction
#[derive(Debug, Clone)]
pub struct RpcSimulateTransactionRequest {
    pub transaction: String, // base64 encoded transaction
    pub config: Option<SimulateTransactionConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulateTransactionConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sig_verify: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_recent_blockhash: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<TransactionEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<SimulateAccountsConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SimulateAccountsConfig {
    pub encoding: TransactionEncoding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
}

impl Serialize for RpcSimulateTransactionRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.transaction)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SimulateTransactionResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<Option<serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub units_consumed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcSimulateTransactionResponse {
    pub value: SimulateTransactionResult,
    pub context: RpcContext,
}

impl RpcRequest for RpcSimulateTransactionRequest {
    fn method_name() -> &'static str {
        "simulateTransaction"
    }
    
    type Response = RpcSimulateTransactionResponse;
}

// getSignatureStatuses
#[derive(Debug, Clone)]
pub struct RpcGetSignatureStatusesRequest {
    pub signatures: Vec<String>,
    pub config: Option<SignatureStatusesConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SignatureStatusesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_transaction_history: Option<bool>,
}

impl Serialize for RpcGetSignatureStatusesRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.signatures)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SignatureStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmations: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetSignatureStatusesResponse {
    pub value: Vec<Option<SignatureStatus>>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetSignatureStatusesRequest {
    fn method_name() -> &'static str {
        "getSignatureStatuses"
    }
    
    type Response = RpcGetSignatureStatusesResponse;
}

// getRecentPrioritizationFees
#[derive(Debug, Clone)]
pub struct RpcGetRecentPrioritizationFeesRequest {
    pub addresses: Option<Vec<String>>,
}

impl Serialize for RpcGetRecentPrioritizationFeesRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = if self.addresses.is_some() { 1 } else { 0 };
        let mut seq = serializer.serialize_seq(Some(len))?;
        if let Some(ref addresses) = self.addresses {
            seq.serialize_element(addresses)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PrioritizationFee {
    pub slot: u64,
    pub prioritization_fee: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetRecentPrioritizationFeesResponse {
    pub value: Vec<PrioritizationFee>,
}

impl RpcRequest for RpcGetRecentPrioritizationFeesRequest {
    fn method_name() -> &'static str {
        "getRecentPrioritizationFees"
    }
    
    type Response = RpcGetRecentPrioritizationFeesResponse;
}


