use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use eth2::types::{BlockId, EthSpec, SignedBeaconBlock, StateId, ValidatorId};
use eth2::BeaconNodeHttpClient;

pub type BeaconClient = BeaconNodeHttpClient;

pub async fn get_current_finality_block<T: EthSpec>(
    beacon: &BeaconNodeHttpClient,
) -> anyhow::Result<SignedBeaconBlock<T>> {
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

pub async fn get_validator_balances_by_slot(
    beacon: &BeaconClient,
    slot: u64,
    validator_ids: &HashSet<u64>,
) -> anyhow::Result<HashMap<u64, u64>> {
    let validator_ids: Vec<_> = validator_ids
        .iter()
        .map(|id| ValidatorId::Index(*id))
        .collect();
    let result = beacon
        .get_beacon_states_validator_balances(
            StateId::Slot(slot.into()),
            Some(validator_ids.as_slice()),
        )
        .await
        .map_err(|err| anyhow!("get validator balance {err}"))?
        .ok_or(anyhow!("{slot} missing"))?;
    let balances = result
        .data
        .into_iter()
        .map(|data| (data.index, data.balance))
        .collect();
    Ok(balances)
}

pub async fn get_beacon_block_by_slot<T: EthSpec>(
    client: &BeaconClient,
    slot: u64,
) -> anyhow::Result<Option<SignedBeaconBlock<T>>> {
    Ok(client
        .get_beacon_blocks(BlockId::Slot(slot.into()))
        .await
        .map_err(|err| anyhow!("get beacon block error {err}"))?
        .map(|response| response.data))
}
