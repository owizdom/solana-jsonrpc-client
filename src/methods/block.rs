use crate::client::RpcRequest;
use crate::types::{Block, BlockEncoding, Commitment, RpcContext};
use serde::{Deserialize, Serialize};

// getBlock
#[derive(Debug, Clone)]
pub struct RpcGetBlockRequest {
    pub slot: u64,
    pub config: Option<BlockConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<BlockEncoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_details: Option<TransactionDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(rename = "maxSupportedTransactionVersion", skip_serializing_if = "Option::is_none")]
    pub max_supported_transaction_version: Option<u8>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionDetails {
    Full,
    Signatures,
    None,
}

impl Serialize for RpcGetBlockRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.slot)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RpcGetBlockResponse {
    Simple(Option<Block>),
    WithContext {
        #[serde(skip_serializing_if = "Option::is_none")]
        value: Option<Block>,
        context: RpcContext,
    },
}

impl RpcRequest for RpcGetBlockRequest {
    fn method_name() -> &'static str {
        "getBlock"
    }
    
    type Response = RpcGetBlockResponse;
}

// getBlockHeight
#[derive(Debug, Clone)]
pub struct RpcGetBlockHeightRequest {
    pub commitment: Option<Commitment>,
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetBlockHeightRequest {
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
#[serde(untagged)]
pub enum RpcGetBlockHeightResponse {
    Simple(u64),
    WithContext {
        value: u64,
        context: RpcContext,
    },
}

impl RpcRequest for RpcGetBlockHeightRequest {
    fn method_name() -> &'static str {
        "getBlockHeight"
    }
    
    type Response = RpcGetBlockHeightResponse;
}

// getSlot
#[derive(Debug, Clone)]
pub struct RpcGetSlotRequest {
    pub commitment: Option<Commitment>,
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetSlotRequest {
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
#[serde(untagged)]
pub enum RpcGetSlotResponse {
    Simple(u64),
    WithContext {
        value: u64,
        context: RpcContext,
    },
}

impl RpcRequest for RpcGetSlotRequest {
    fn method_name() -> &'static str {
        "getSlot"
    }
    
    type Response = RpcGetSlotResponse;
}

// getSlotLeader
#[derive(Debug, Clone)]
pub struct RpcGetSlotLeaderRequest {
    pub slot: Option<u64>,
    pub commitment: Option<Commitment>,
}

impl Serialize for RpcGetSlotLeaderRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = (if self.slot.is_some() { 1 } else { 0 }) + (if self.commitment.is_some() { 1 } else { 0 });
        let mut seq = serializer.serialize_seq(Some(len))?;
        if let Some(ref slot) = self.slot {
            seq.serialize_element(slot)?;
        }
        if let Some(ref commitment) = self.commitment {
            seq.serialize_element(commitment)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetSlotLeaderResponse {
    pub value: String,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetSlotLeaderRequest {
    fn method_name() -> &'static str {
        "getSlotLeader"
    }
    
    type Response = RpcGetSlotLeaderResponse;
}

// getLatestBlockhash
#[derive(Debug, Clone)]
pub struct RpcGetLatestBlockhashRequest {
    pub commitment: Option<Commitment>,
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetLatestBlockhashRequest {
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
pub struct BlockhashValue {
    pub blockhash: String,
    pub last_valid_block_height: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetLatestBlockhashResponse {
    pub value: BlockhashValue,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetLatestBlockhashRequest {
    fn method_name() -> &'static str {
        "getLatestBlockhash"
    }
    
    type Response = RpcGetLatestBlockhashResponse;
}

// isBlockhashValid
#[derive(Debug, Clone)]
pub struct RpcIsBlockhashValidRequest {
    pub blockhash: String,
    pub config: Option<BlockhashValidConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockhashValidConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcIsBlockhashValidRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.blockhash)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcIsBlockhashValidResponse {
    pub value: bool,
    pub context: RpcContext,
}

impl RpcRequest for RpcIsBlockhashValidRequest {
    fn method_name() -> &'static str {
        "isBlockhashValid"
    }
    
    type Response = RpcIsBlockhashValidResponse;
}

// getBlocks
#[derive(Debug, Clone)]
pub struct RpcGetBlocksRequest {
    pub start_slot: u64,
    pub end_slot: Option<u64>,
    pub commitment: Option<Commitment>,
}

impl Serialize for RpcGetBlocksRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = 1 + if self.end_slot.is_some() { 1 } else { 0 } + if self.commitment.is_some() { 1 } else { 0 };
        let mut seq = serializer.serialize_seq(Some(len))?;
        seq.serialize_element(&self.start_slot)?;
        if let Some(ref end_slot) = self.end_slot {
            seq.serialize_element(end_slot)?;
        }
        if let Some(ref commitment) = self.commitment {
            seq.serialize_element(commitment)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetBlocksResponse {
    pub value: Vec<u64>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetBlocksRequest {
    fn method_name() -> &'static str {
        "getBlocks"
    }
    
    type Response = RpcGetBlocksResponse;
}

// getFirstAvailableBlock
#[derive(Debug, Clone)]
pub struct RpcGetFirstAvailableBlockRequest;

impl Serialize for RpcGetFirstAvailableBlockRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let seq = serializer.serialize_seq(Some(0))?;
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetFirstAvailableBlockResponse {
    pub value: u64,
}

impl RpcRequest for RpcGetFirstAvailableBlockRequest {
    fn method_name() -> &'static str {
        "getFirstAvailableBlock"
    }
    
    type Response = RpcGetFirstAvailableBlockResponse;
}

