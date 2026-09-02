use alloy_eips::BlockNumberOrTag;
use alloy_network::TransactionResponse;
use alloy_primitives::Address;
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

pub async fn latest_block_first_transaction(
    rpc_url: &str,
) -> Result<(String, String, u64), Box<dyn std::error::Error + Send + Sync>> {
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    let transaction = provider
        .get_transaction_by_block_number_and_index(BlockNumberOrTag::Latest, 0)
        .await?
        .ok_or("no transaction found in latest block")?;

    let tx_hash = transaction.tx_hash().to_string();
    let from = transaction.from().to_string();
    let block_number = transaction
        .block_number()
        .ok_or("transaction has no block number")?;

    Ok((tx_hash, from, block_number))
}

pub async fn account_balance(
    rpc_url: &str,
    address: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    let address: Address = address.parse()?;
    let balance = provider.get_balance(address).await?;

    Ok(balance.to_string())
}
