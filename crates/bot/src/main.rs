use polygon_rpc::{
    account_balance, latest_block_first_transaction, latest_block_info, polygon_status,
    usdc_decimals,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let rpc_url = env::var("POLYGON_HTTP_RPC_URL")?;

    println!("Connecting to Polygon PoS...");

    let (chain_id, block_number) = polygon_status(&rpc_url).await?;

    println!("Chain ID: {chain_id}");
    println!("Latest block: {block_number}");

    let (number, hash) = latest_block_info(&rpc_url).await?;

    println!("Block number: {number}");
    println!("Block hash: {hash}");

    let (tx_hash, from, tx_block) = latest_block_first_transaction(&rpc_url).await?;

    println!("Transaction hash: {tx_hash}");
    println!("Transaction from: {from}");
    println!("Transaction block: {tx_block}");

    let balance = account_balance(&rpc_url, &from).await?;

    println!("Account balance (base units): {balance}");

    let decimals = usdc_decimals(&rpc_url).await?;

    println!("USDC decimals: {decimals}");

    Ok(())
}
