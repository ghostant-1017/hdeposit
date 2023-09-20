use anyhow::anyhow;
use eth2::types::{BeaconBlock, EthSpec, Hash256};
use ethers::types::{Block as Eth1Block, Transaction, TransactionReceipt};

pub async fn extract_el_rewards_capella<T: EthSpec>(block: BeaconBlock<T>) -> anyhow::Result<()> {
    let block = block.as_capella().map_err(|_| anyhow!("not capella block"))?;
    let proposer_index = block.proposer_index;
    // 1. ValidatorStore.contains(proposer_index)?;
    
    // 2. Retrieve fee recipient and block hash
    let fee_recipient = block.body.execution_payload.execution_payload.fee_recipient;
    let block_hash = block.body.execution_payload.execution_payload.block_hash;

    // 3. query block_hash from eth1
    todo!()
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