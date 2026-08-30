use polygon_rpc::polygon_status;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let rpc_url = env::var("POLYGON_HTTP_RPC_URL")?;

    println!("Connecting to Polygon PoS...");

    let (chain_id, block_number) = polygon_status(&rpc_url).await?;

    println!("Chain ID: {chain_id}");
    println!("Latest block: {block_number}");

    Ok(())
}
