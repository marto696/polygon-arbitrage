use alloy_eips::BlockNumberOrTag;
use alloy_provider::{Provider, ProviderBuilder};

pub async fn polygon_status(
    rpc_url: &str,
) -> Result<(u64, u64), Box<dyn std::error::Error + Send + Sync>> {
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    let chain_id = provider.get_chain_id().await?;
    let block_number = provider.get_block_number().await?;

    Ok((chain_id, block_number))
}

pub async fn latest_block_info(
    rpc_url: &str,
) -> Result<(u64, String), Box<dyn std::error::Error + Send + Sync>> {
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    let block = provider
        .get_block_by_number(BlockNumberOrTag::Latest)
        .await?
        .ok_or("latest block not found")?;

    Ok((block.header.number, block.header.hash.to_string()))
}
