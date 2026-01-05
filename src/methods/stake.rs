use crate::client::RpcRequest;
use crate::types::RpcContext;
use serde::{Deserialize, Serialize};

// getStakeActivation
#[derive(Debug, Clone)]
pub struct RpcGetStakeActivationRequest {
    pub pubkey: String,
    pub config: Option<StakeActivationConfig>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StakeActivationConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

impl Serialize for RpcGetStakeActivationRequest {
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

#[derive(Debug, Clone, Deserialize)]
pub struct StakeActivation {
    pub state: String,
    pub active: u64,
    pub inactive: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcGetStakeActivationResponse {
    pub value: StakeActivation,
    pub context: RpcContext,
}

impl RpcRequest for RpcGetStakeActivationRequest {
    fn method_name() -> &'static str {
        "getStakeActivation"
    }
    
    type Response = RpcGetStakeActivationResponse;
}

