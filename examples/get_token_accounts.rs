use solana_jsonrpc_client::{methods, JsonRpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://api.mainnet-beta.solana.com");

    // Example: Get token accounts for a wallet (replace with a real wallet address)
    let owner = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";

    let request = methods::token::RpcGetTokenAccountsByOwnerRequest {
        owner: owner.to_string(),
        config: methods::token::TokenAccountsConfig {
            commitment: Some(solana_jsonrpc_client::types::Commitment::Finalized),
            encoding: Some(solana_jsonrpc_client::types::Encoding::JsonParsed),
            mint: None,
            program_id: None,
            min_context_slot: None,
        },
    };

    let response = client.call(request).await?;
    
    println!("Token Accounts Response:");
    println!("  Context Slot: {}", response.context.slot);
    println!("  Number of token accounts: {}", response.value.len());
    
    for (i, account) in response.value.iter().take(5).enumerate() {
        println!("  Token Account {}: {}", i + 1, account.pubkey);
    }

    Ok(())
}

