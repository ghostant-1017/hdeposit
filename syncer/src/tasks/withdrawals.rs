use anyhow::anyhow;
use eth2::types::{BeaconBlock, EthSpec};
use ethers::types::H160;

pub struct ValidatorWithdrawal {
    slot: u64,
    index: u64,
    validator_index: u64,
    address: H160,
    amount: u64
}

pub fn extract_withdrawals_capella<T: EthSpec>(block: &BeaconBlock<T>) -> anyhow::Result<()> {
    let block = block.as_capella().map_err(|_| anyhow!("not capella block"))?;
    let withdrawals = block.body.execution_payload.execution_payload.withdrawals.to_vec();

    todo!()
}