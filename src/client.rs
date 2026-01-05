use crate::error::{JsonRpcError, Result};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static REQUEST_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
pub struct JsonRpcClient {
    url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct JsonRpcResponse<T> {
    #[allow(dead_code)]
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcErrorResponse>,
    #[allow(dead_code)]
    id: u64,
}

#[derive(Debug, Deserialize)]
struct JsonRpcErrorResponse {
    code: i64,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[allow(dead_code)]
    data: Option<serde_json::Value>,
}

pub trait RpcRequest: Serialize {
    fn method_name() -> &'static str;
    type Response: for<'de> Deserialize<'de>;
}

impl JsonRpcClient {
    pub fn connect(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn call<R>(&self, request: R) -> Result<R::Response>
    where
        R: RpcRequest,
    {
        let id = REQUEST_ID.fetch_add(1, Ordering::Relaxed);
        let params = serde_json::to_value(&request)?;
        let jsonrpc_request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id,
            method: R::method_name().to_string(),
            params,
        };

        let response = self
            .client
            .post(&self.url)
            .json(&jsonrpc_request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            return Err(JsonRpcError::InvalidResponse(format!(
                "HTTP error: {}",
                status
            )));
        }

        let jsonrpc_response: JsonRpcResponse<R::Response> = response.json().await?;

        if let Some(error) = jsonrpc_response.error {
            return Err(JsonRpcError::Rpc {
                code: error.code,
                message: error.message,
            });
        }

        jsonrpc_response
            .result
            .ok_or(JsonRpcError::MissingResult)
    }
}

