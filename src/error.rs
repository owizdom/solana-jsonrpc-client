use thiserror::Error;

#[derive(Error, Debug)]
pub enum JsonRpcError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON-RPC error: code={code}, message={message}")]
    Rpc { code: i64, message: String },

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] serde_json::Error),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Missing result in response")]
    MissingResult,
}

pub type Result<T> = std::result::Result<T, JsonRpcError>;


