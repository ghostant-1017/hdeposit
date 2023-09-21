use anyhow::anyhow;
use eth2::BeaconNodeHttpClient;
use eth2::types::{EthSpec, BlockId, SignedBeaconBlock};

pub type BeaconClient = BeaconNodeHttpClient;

pub async fn get_current_finality_block<T: EthSpec>(beacon: &BeaconNodeHttpClient) -> anyhow::Result<SignedBeaconBlock<T>>{
    let response = beacon
        .get_beacon_blocks::<T>(BlockId::Finalized)
        .await
        .map_err(|err| anyhow!("{err}"))?
        .ok_or(anyhow!("block number not found"))?;
    let block = response.data;
    Ok(block)
}

pub async fn get_current_finality_block_number<T: EthSpec>(
    beacon: &BeaconNodeHttpClient,
) -> anyhow::Result<u64> {
    let block = get_current_finality_block::<T>(beacon).await?;
    let payload = block
        .message()
        .execution_payload()
        .map_err(|err| anyhow!("{err:?}"))?;
    let block_number = payload.execution_payload_ref().block_number();
    Ok(block_number)
}