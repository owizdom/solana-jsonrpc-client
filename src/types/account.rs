use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: String,
    pub executable: bool,
    pub rent_epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountInfo {
    pub lamports: u64,
    #[serde(rename = "data")]
    pub data_encoded: AccountData,
    pub owner: String,
    pub executable: bool,
    #[serde(rename = "rentEpoch")]
    pub rent_epoch: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccountData {
    // Array format: [data_string, encoding]
    Array(Vec<serde_json::Value>),
    // String format (base58/base64)
    String(String),
    // Object format (jsonParsed)
    Object(serde_json::Value),
}

