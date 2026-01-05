use solana_jsonrpc_client::{methods, JsonRpcClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = JsonRpcClient::connect("https://api.mainnet-beta.solana.com");

    // First, get the current slot
    let slot_request = methods::block::RpcGetSlotRequest {
        commitment: Some(solana_jsonrpc_client::types::Commitment::Finalized),
        min_context_slot: None,
    };
    let slot_response = client.call(slot_request).await?;
    let current_slot = match slot_response {
        methods::block::RpcGetSlotResponse::Simple(slot) => slot,
        methods::block::RpcGetSlotResponse::WithContext { value, .. } => value,
    };
    println!("Current slot: {}", current_slot);

    // Get a recent block
    let request = methods::block::RpcGetBlockRequest {
        slot: current_slot - 10, // Get a block from 10 slots ago
        config: Some(methods::block::BlockConfig {
            encoding: Some(solana_jsonrpc_client::types::BlockEncoding::Json),
            transaction_details: Some(methods::block::TransactionDetails::Signatures),
            rewards: Some(true),
            commitment: Some(solana_jsonrpc_client::types::Commitment::Finalized),
            max_supported_transaction_version: Some(0),
        }),
    };

    let response = client.call(request).await?;
    
    println!("Block Response:");
    let (block, context_slot) = match &response {
        methods::block::RpcGetBlockResponse::Simple(block) => (block.as_ref(), None),
        methods::block::RpcGetBlockResponse::WithContext { value, context } => {
            (value.as_ref(), Some(context.slot))
        }
    };
    if let Some(slot) = context_slot {
        println!("  Context Slot: {}", slot);
    }
    if let Some(block) = block {
        if let Some(ref blockhash) = block.blockhash {
            println!("  Blockhash: {}", blockhash);
        }
        if let Some(ref previous_blockhash) = block.previous_blockhash {
            println!("  Previous Blockhash: {}", previous_blockhash);
        }
        if let Some(ref transactions) = block.transactions {
            println!("  Transactions: {}", transactions.len());
        }
        if let Some(ref block_time) = block.block_time {
            println!("  Block Time: {}", block_time);
        }
    } else {
        println!("  Block not found");
    }

    Ok(())
}

