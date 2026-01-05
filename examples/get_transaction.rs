use solana_jsonrpc_client::{methods, JsonRpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://api.mainnet-beta.solana.com");

    // Example transaction signature (replace with a real one)
    let signature = "5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW";

    let request = methods::transaction::RpcGetTransactionRequest {
        signature: signature.to_string(),
        config: Some(methods::transaction::TransactionConfig {
            encoding: Some(solana_jsonrpc_client::types::TransactionEncoding::Json),
            commitment: Some(solana_jsonrpc_client::types::Commitment::Finalized),
            max_supported_transaction_version: None,
        }),
    };

    let response = client.call(request).await?;
    
    println!("Transaction Response:");
    println!("  Context Slot: {}", response.context.slot);
    if let Some(transaction) = response.value {
        if let Some(ref slot) = transaction.slot {
            println!("  Slot: {}", slot);
        }
        if let Some(ref block_time) = transaction.block_time {
            println!("  Block Time: {}", block_time);
        }
        println!("  Transaction data available");
    } else {
        println!("  Transaction not found");
    }

    Ok(())
}

