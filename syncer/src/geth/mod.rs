use anyhow::Result;
use ethers::providers::Http;
use ethers::providers::Middleware;
use ethers::providers::Provider;
use ethers::types::BlockNumber;
use ethers::types::TransactionReceipt;
pub type Eth1Client = Provider<Http>;

pub async fn get_block_receipts_by_hash(
    client: &Eth1Client,
    block_number: u64,
) -> Result<Vec<TransactionReceipt>> {
    let block_receipt = client
        .get_block_receipts(BlockNumber::Number(block_number.into()))
        .await?;
    Ok(block_receipt)
}
