use crate::client::RpcRequest;
use crate::types::{Commitment, RpcContext};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// getEpochInfo
#[derive(Debug, Clone)]
pub struct RpcGetEpochInfoRequest {
    pub commitment: Option<Commitment>,
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetEpochInfoRequest {
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
pub struct EpochInfo {
    pub absolute_slot: u64,
    pub block_height: u64,
    pub epoch: u64,
    pub slot_index: u64,
    pub slots_in_epoch: u64,
    pub transaction_count: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetEpochInfoResponse {
    pub value: EpochInfo,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetEpochInfoRequest {
    fn method_name() -> &'static str {
        "getEpochInfo"
    }
    
    type Response = RpcGetEpochInfoResponse;
}

// getEpochSchedule
#[derive(Debug, Clone)]
pub struct RpcGetEpochScheduleRequest;

impl Serialize for RpcGetEpochScheduleRequest {
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
pub struct EpochSchedule {
    pub slots_per_epoch: u64,
    pub leader_schedule_slot_offset: u64,
    pub warmup: bool,
    pub first_normal_epoch: u64,
    pub first_normal_slot: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetEpochScheduleResponse {
    pub value: EpochSchedule,
}

impl RpcRequest for RpcGetEpochScheduleRequest {
    fn method_name() -> &'static str {
        "getEpochSchedule"
    }
    
    type Response = RpcGetEpochScheduleResponse;
}

// getLeaderSchedule
#[derive(Debug, Clone)]
pub struct RpcGetLeaderScheduleRequest {
    pub slot: Option<u64>,
    pub config: Option<LeaderScheduleConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LeaderScheduleConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
}

impl Serialize for RpcGetLeaderScheduleRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let len = (if self.slot.is_some() { 1 } else { 0 }) + (if self.config.is_some() { 1 } else { 0 });
        let mut seq = serializer.serialize_seq(Some(len))?;
        if let Some(ref slot) = self.slot {
            seq.serialize_element(slot)?;
        }
        if let Some(ref config) = self.config {
            seq.serialize_element(config)?;
        }
        seq.end()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetLeaderScheduleResponse {
    pub value: HashMap<String, Vec<u64>>,
}

impl RpcRequest for RpcGetLeaderScheduleRequest {
    fn method_name() -> &'static str {
        "getLeaderSchedule"
    }
    
    type Response = RpcGetLeaderScheduleResponse;
}

