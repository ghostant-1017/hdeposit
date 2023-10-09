use anyhow::Result;
use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::types::Block;
use ethers::types::BlockNumber;
use ethers::types::Transaction;
use ethers::types::TransactionReceipt;
pub type Eth1Client = Provider<Http>;

pub async fn get_block_transactions_by_number(
    client: &Eth1Client,
    block_number: u64,
) -> Result<Option<Block<Transaction>>> {
    let block = client
        .get_block_with_txs(BlockNumber::Number(block_number.into()))
        .await?;
    Ok(block)
}

pub async fn get_block_receipts_by_number(
    client: &Eth1Client,
    block_number: u64
) -> Result<Vec<TransactionReceipt>> {
    let receipts = client
    .get_block_receipts(BlockNumber::Number(block_number.into()))
    .await?;
    Ok(receipts)
}