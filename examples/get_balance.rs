use solana_jsonrpc_client::{methods, JsonRpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://api.mainnet-beta.solana.com");

    let request = methods::account::RpcGetBalanceRequest {
        pubkey: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
        config: Some(methods::account::BalanceConfig {
            commitment: Some(solana_jsonrpc_client::types::Commitment::Finalized),
            min_context_slot: None,
        }),
    };

    let response = client.call(request).await?;
    
    println!("Balance: {} lamports", response.value);
    println!("Context Slot: {}", response.context.slot);
    println!("Balance in SOL: {:.9}", response.value as f64 / 1_000_000_000.0);

    Ok(())
}

