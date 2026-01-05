use solana_jsonrpc_client::{methods, JsonRpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://api.mainnet-beta.solana.com");

    // Example: Get accounts for the System Program
    let program_id = "11111111111111111111111111111111";

    let request = methods::program::RpcGetProgramAccountsRequest {
        program_id: program_id.to_string(),
        config: Some(methods::program::ProgramAccountsConfig {
            commitment: Some(solana_jsonrpc_client::types::Commitment::Finalized),
            encoding: Some(solana_jsonrpc_client::types::Encoding::Base64),
            filters: None,
            min_context_slot: None,
            data_slice: None,
            with_context: None,
        }),
    };

    let response = client.call(request).await?;
    
    println!("Program Accounts Response:");
    println!("  Context Slot: {}", response.context.slot);
    println!("  Number of accounts: {}", response.value.len());
    
    // Print first few accounts
    for (i, account) in response.value.iter().take(5).enumerate() {
        println!("  Account {}: {}", i + 1, account.pubkey);
        println!("    Lamports: {}", account.account.lamports);
    }

    Ok(())
}

