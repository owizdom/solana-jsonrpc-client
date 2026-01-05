use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlockEncoding {
    Json,
    JsonParsed,
    Binary,
    Base64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub blockhash: Option<String>,
    pub previous_blockhash: Option<String>,
    pub parent_slot: Option<u64>,
    pub transactions: Option<Vec<serde_json::Value>>,
    pub rewards: Option<Vec<serde_json::Value>>,
    pub block_time: Option<i64>,
    pub block_height: Option<u64>,
}


