use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Encoding {
    Base58,
    Base64,
    JsonParsed,
    #[serde(rename = "json")]
    Json,
}


