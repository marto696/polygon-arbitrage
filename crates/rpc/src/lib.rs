use alloy_eips::BlockNumberOrTag;
use alloy_network::TransactionResponse;

use alloy_primitives::{Address, Bytes, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types_eth::{TransactionInput, TransactionRequest};

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

pub async fn usdc_decimals(rpc_url: &str) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    let usdc_address: Address = "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359".parse()?;

    // decimals() selector = 0x313ce567
    let call_data = Bytes::from_static(&[0x31, 0x3c, 0xe5, 0x67]);

    let request = TransactionRequest::default()
        .to(usdc_address)
        .input(TransactionInput::new(call_data));

    let result = provider.call(request).await?;

    let decimals = U256::from_be_slice(&result).to::<u64>();

    Ok(decimals)
}

pub async fn usdc_decimals_gas_estimate(
    rpc_url: &str,
) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let provider = ProviderBuilder::new().connect(rpc_url).await?;

    let usdc_address: Address = "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359".parse()?;

    // decimals() selector = 0x313ce567
    let call_data = Bytes::from_static(&[0x31, 0x3c, 0xe5, 0x67]);

    let request = TransactionRequest::default()
        .to(usdc_address)
        .input(TransactionInput::new(call_data));

    let gas = provider.estimate_gas(request).await?;

    Ok(gas)
}
