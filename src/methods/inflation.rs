use crate::client::RpcRequest;
use crate::types::{Commitment, RpcContext};
use serde::{Deserialize, Serialize};

// getInflationGovernor
#[derive(Debug, Clone)]
pub struct RpcGetInflationGovernorRequest {
    pub commitment: Option<Commitment>,
}

impl Serialize for RpcGetInflationGovernorRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = if self.commitment.is_some() { 1 } else { 0 };
        let mut seq = serializer.serialize_seq(Some(len))?;
        if let Some(ref commitment) = self.commitment {
            seq.serialize_element(commitment)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InflationGovernor {
    pub initial: f64,
    pub terminal: f64,
    pub taper: f64,
    pub foundation: f64,
    pub foundation_term: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetInflationGovernorResponse {
    pub value: InflationGovernor,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetInflationGovernorRequest {
    fn method_name() -> &'static str {
        "getInflationGovernor"
    }
    
    type Response = RpcGetInflationGovernorResponse;
}

// getInflationRate
#[derive(Debug, Clone)]
pub struct RpcGetInflationRateRequest;

impl Serialize for RpcGetInflationRateRequest {
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
pub struct InflationRate {
    pub total: f64,
    pub validator: f64,
    pub foundation: f64,
    pub epoch: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetInflationRateResponse {
    pub value: InflationRate,
}

impl RpcRequest for RpcGetInflationRateRequest {
    fn method_name() -> &'static str {
        "getInflationRate"
    }
    
    type Response = RpcGetInflationRateResponse;
}

// getInflationReward
#[derive(Debug, Clone)]
pub struct RpcGetInflationRewardRequest {
    pub addresses: Vec<String>,
    pub config: Option<InflationRewardConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InflationRewardConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetInflationRewardRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(if self.config.is_some() { 2 } else { 1 }))?;
        seq.serialize_element(&self.addresses)?;
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InflationReward {
    pub epoch: u64,
    pub effective_slot: u64,
    pub amount: u64,
    pub post_balance: u64,
    pub commission: Option<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetInflationRewardResponse {
    pub value: Vec<Option<InflationReward>>,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetInflationRewardRequest {
    fn method_name() -> &'static str {
        "getInflationReward"
    }
    
    type Response = RpcGetInflationRewardResponse;
}


