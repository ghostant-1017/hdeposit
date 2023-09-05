use anyhow::{anyhow, Result};
use eth2::{BeaconNodeHttpClient, types::{ValidatorData, StateId}};
use storage::models::query_used_keystore;

pub async fn update_validators(beacon: &BeaconNodeHttpClient) -> Result<Vec<ValidatorData>> {
    let validators = beacon
        .get_beacon_states_validators(StateId::Finalized, None, None)
        .await
        .map_err(|err| anyhow!("{err}"))?
        .unwrap();
    Ok(validators.data)
}
