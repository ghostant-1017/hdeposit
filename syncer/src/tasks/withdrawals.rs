use eth2::types::{BeaconBlock, EthSpec};


pub fn extract_withdrawals_capella<T: EthSpec>(beacon: &BeaconBlock<T>) -> anyhow::Result<()> {
    todo!()
}