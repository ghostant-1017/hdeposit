use anyhow::anyhow;
use crate::geth::Eth1Client;
use crate::geth::get_block_receipts_by_hash;
use storage::models::ELFee;
use eth2::types::{BeaconBlock, EthSpec};
use ethers::types::TransactionReceipt;


pub async fn extract_el_rewards_capella<T: EthSpec>(block: BeaconBlock<T>, eth1: &Eth1Client) -> anyhow::Result<ELFee> {
    let block = block.as_capella().map_err(|_| anyhow!("not capella block"))?;
    let slot = block.slot.as_u64();
    let validator_index = block.proposer_index;

    let fee_recipient = block.body.execution_payload.execution_payload.fee_recipient;
    let block_number = block.body.execution_payload.execution_payload.block_number;
    let block_hash = block.body.execution_payload.execution_payload.block_hash;
    // 3. query block_hash from eth1
    let receipts = get_block_receipts_by_hash(eth1, block_number).await?;
    let amount = caculate_block_fee(receipts)?;
    Ok(ELFee {
        slot,
        block_number,
        block_hash: block_hash.into_root(),
        validator_index,
        fee_recipient,
        amount
    })
}

fn caculate_block_fee(receipts: Vec<TransactionReceipt>) -> anyhow::Result<u64> {
    let mut total = 0;
    for receipt in receipts {
        let gas_price = receipt.effective_gas_price.ok_or(anyhow!("effective_gas_price not found"))?.as_u64();
        let gas_used = receipt.gas_used.ok_or(anyhow!("gas_used not found"))?.as_u64();
        total += gas_price * gas_used;
    }
    Ok(total)
}