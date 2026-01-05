use crate::client::RpcRequest;
use crate::types::{Commitment, RpcContext};
use serde::{Deserialize, Serialize};

// getVersion
#[derive(Debug, Clone)]
pub struct RpcGetVersionRequest;

impl Serialize for RpcGetVersionRequest {
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
pub struct VersionInfo {
    #[serde(rename = "solana-core")]
    pub solana_core: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_set: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetVersionResponse {
    pub value: VersionInfo,
}

impl RpcRequest for RpcGetVersionRequest {
    fn method_name() -> &'static str {
        "getVersion"
    }
    
    type Response = RpcGetVersionResponse;
}

// getHealth
#[derive(Debug, Clone)]
pub struct RpcGetHealthRequest;

impl Serialize for RpcGetHealthRequest {
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
#[serde(untagged)]
pub enum RpcGetHealthResponse {
    Ok(String),
    Error { code: i64, message: String },
}

impl RpcRequest for RpcGetHealthRequest {
    fn method_name() -> &'static str {
        "getHealth"
    }
    
    type Response = RpcGetHealthResponse;
}

// getClusterNodes
#[derive(Debug, Clone)]
pub struct RpcGetClusterNodesRequest;

impl Serialize for RpcGetClusterNodesRequest {
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
pub struct ClusterNode {
    pub pubkey: String,
    pub gossip: Option<String>,
    pub tpu: Option<String>,
    pub rpc: Option<String>,
    pub version: Option<String>,
    pub feature_set: Option<u64>,
    pub shred_version: Option<u16>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetClusterNodesResponse {
    pub value: Vec<ClusterNode>,
}

impl RpcRequest for RpcGetClusterNodesRequest {
    fn method_name() -> &'static str {
        "getClusterNodes"
    }
    
    type Response = RpcGetClusterNodesResponse;
}

// getGenesisHash
#[derive(Debug, Clone)]
pub struct RpcGetGenesisHashRequest;

impl Serialize for RpcGetGenesisHashRequest {
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
pub struct RpcGetGenesisHashResponse {
    pub value: String,
}

impl RpcRequest for RpcGetGenesisHashRequest {
    fn method_name() -> &'static str {
        "getGenesisHash"
    }
    
    type Response = RpcGetGenesisHashResponse;
}

// getIdentity
#[derive(Debug, Clone)]
pub struct RpcGetIdentityRequest;

impl Serialize for RpcGetIdentityRequest {
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
pub struct RpcGetIdentityResponse {
    pub value: IdentityInfo,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IdentityInfo {
    pub identity: String,
}

impl RpcRequest for RpcGetIdentityRequest {
    fn method_name() -> &'static str {
        "getIdentity"
    }
    
    type Response = RpcGetIdentityResponse;
}

// getSupply
#[derive(Debug, Clone)]
pub struct RpcGetSupplyRequest {
    pub commitment: Option<Commitment>,
    pub exclude_non_circulating_supply_list: Option<bool>,
}

impl Serialize for RpcGetSupplyRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = if self.commitment.is_some() || self.exclude_non_circulating_supply_list.is_some() { 1 } else { 0 };
        let mut seq = serializer.serialize_seq(Some(len))?;
        if self.commitment.is_some() || self.exclude_non_circulating_supply_list.is_some() {
            #[derive(Serialize)]
            struct Config {
                #[serde(skip_serializing_if = "Option::is_none")]
                commitment: Option<Commitment>,
                #[serde(skip_serializing_if = "Option::is_none")]
                exclude_non_circulating_supply_list: Option<bool>,
            }
            seq.serialize_element(&Config {
                commitment: self.commitment,
                exclude_non_circulating_supply_list: self.exclude_non_circulating_supply_list,
            })?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SupplyInfo {
    pub total: u64,
    pub circulating: u64,
    pub non_circulating: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_circulating_accounts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetSupplyResponse {
    pub value: SupplyInfo,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetSupplyRequest {
    fn method_name() -> &'static str {
        "getSupply"
    }
    
    type Response = RpcGetSupplyResponse;
}

// getMaxRetransmitSlot
#[derive(Debug, Clone)]
pub struct RpcGetMaxRetransmitSlotRequest;

impl Serialize for RpcGetMaxRetransmitSlotRequest {
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
pub struct RpcGetMaxRetransmitSlotResponse {
    pub value: u64,
}

impl RpcRequest for RpcGetMaxRetransmitSlotRequest {
    fn method_name() -> &'static str {
        "getMaxRetransmitSlot"
    }
    
    type Response = RpcGetMaxRetransmitSlotResponse;
}

// getMaxShredInsertSlot
#[derive(Debug, Clone)]
pub struct RpcGetMaxShredInsertSlotRequest;

impl Serialize for RpcGetMaxShredInsertSlotRequest {
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
pub struct RpcGetMaxShredInsertSlotResponse {
    pub value: u64,
}

impl RpcRequest for RpcGetMaxShredInsertSlotRequest {
    fn method_name() -> &'static str {
        "getMaxShredInsertSlot"
    }
    
    type Response = RpcGetMaxShredInsertSlotResponse;
}


