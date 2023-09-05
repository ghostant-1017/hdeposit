use lighthouse_types::EthSpec;
use std::{str::FromStr, time::Duration};
use eth2::{BeaconNodeHttpClient, SensitiveUrl, Timeouts, types::{StateId, BlockId, ValidatorData}};
use anyhow::{Result, anyhow};

pub fn new_eth2_client(eth2_enpoint: &str) -> Result<BeaconNodeHttpClient> {
    let url = SensitiveUrl::parse(eth2_enpoint).map_err(|_| anyhow::anyhow!("Parse eth2 endpoint error"))?;
    Ok(BeaconNodeHttpClient::new(url, Timeouts::set_all(Duration::from_secs(5))))
}

pub async fn get_current_finality_block_number<T: EthSpec>(client: &BeaconNodeHttpClient) -> Result<u64> {
    let block = client
    .get_beacon_blocks::<T>(BlockId::Finalized)
    .await
    .map_err(|err| anyhow!("{err}"))?
    .unwrap();
    let payload = block.data
    .message()
    .execution_payload()
    .map_err(|err| anyhow!("{err:?}"))?;
    let block_number = payload.execution_payload_ref().block_number();
    Ok(block_number)
}

pub async fn get_validators<T: EthSpec>(client: &BeaconNodeHttpClient) -> Result<Vec<ValidatorData>> {
    let validators = client
    .get_beacon_states_validators(StateId::Finalized, None, None)
    .await
    .map_err(|err| anyhow!("{err}"))?
    .unwrap();
    Ok(validators.data)
}
