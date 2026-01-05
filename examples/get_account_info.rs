use solana_jsonrpc_client::{methods, JsonRpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://api.mainnet-beta.solana.com");

    let request = methods::account::RpcGetAccountInfoRequest {
        pubkey: "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM".to_string(),
        config: Some(methods::account::AccountInfoConfig {
            encoding: Some(solana_jsonrpc_client::types::Encoding::Base58),
            commitment: Some(solana_jsonrpc_client::types::Commitment::Finalized),
            data_slice: None,
            min_context_slot: None,
        }),
    };

    let response = client.call(request).await?;
    
    println!("Account Info Response:");
    println!("  Context Slot: {}", response.context.slot);
    if let Some(account) = response.value {
        println!("  Lamports: {}", account.lamports);
        println!("  Owner: {}", account.owner);
        println!("  Executable: {}", account.executable);
        println!("  Rent Epoch: {}", account.rent_epoch);
    } else {
        println!("  Account not found");
    }

    Ok(())
}

