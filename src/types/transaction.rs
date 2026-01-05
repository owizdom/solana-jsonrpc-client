use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionEncoding {
    Json,
    JsonParsed,
    Binary,
    Base58,
    Base64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub slot: Option<u64>,
    pub transaction: Option<serde_json::Value>,
    pub meta: Option<serde_json::Value>,
    pub block_time: Option<i64>,
}


