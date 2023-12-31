use anyhow::{anyhow, Result};
use eth2::{types::BlockId, BeaconNodeHttpClient, SensitiveUrl, Timeouts};
use lighthouse_types::EthSpec;
use std::time::Duration;

pub fn new_beacon_client(eth2_enpoint: &str) -> Result<BeaconNodeHttpClient> {
    let url = SensitiveUrl::parse(eth2_enpoint)
        .map_err(|_| anyhow::anyhow!("Parse eth2 endpoint error"))?;
    Ok(BeaconNodeHttpClient::new(
        url,
        Timeouts::set_all(Duration::from_secs(5)),
    ))
}

pub async fn get_current_finality_block_number<T: EthSpec>(
    beacon: &BeaconNodeHttpClient,
) -> Result<u64> {
    let block = beacon
        .get_beacon_blocks::<T>(BlockId::Finalized)
        .await
        .map_err(|err| anyhow!("{err}"))?
        .ok_or(anyhow!("block number not found"))?;
    let payload = block
        .data
        .message()
        .execution_payload()
        .map_err(|err| anyhow!("{err:?}"))?;
    let block_number = payload.execution_payload_ref().block_number();
    Ok(block_number)
}
