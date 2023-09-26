use std::collections::HashSet;

use super::*;
use anyhow::{anyhow, Result};
use bb8_postgres::tokio_postgres::Client;
use eth2::types::{
    fork_versioned_response::ExecutionOptimisticFinalizedForkVersionedResponse, BlockId,
    SignedBeaconBlock,
};
use futures::StreamExt;
use storage::models::{
    select_all_validators, select_sync_state, upsert_sync_state, upsert_withdrawals, SyncState,
};
use tokio::sync::Mutex;
use tracing::info;

impl<T: EthSpec> Updater<T> {
    pub async fn update_withdrawals(&self) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let tx = conn.transaction().await?;
        // 1.Fetch last synced withdrawals slot
        let finalized_slot =
            select_sync_state(tx.client(), &SyncState::WithdrawalFinalizedSlot).await?;
        let start = finalized_slot.unwrap_or(self.start);

        let validator_indexes: HashSet<_> = select_all_validators(tx.client())
            .await?
            .into_iter()
            .map(|validator| validator.index)
            .filter(|index| index.is_some())
            .map(|index| index.unwrap())
            .collect();
        // 2.Query finalized slot
        let current_finalized = self
            .beacon
            .get_beacon_blocks::<T>(BlockId::Finalized)
            .await
            .map_err(|err| anyhow!("{err}"))?
            .unwrap();
        let end = current_finalized.data.slot().as_u64();
        // 3.Query [start,end] blocks
        info!("Update withdrawals from {start} to {end}");
        let err_ctx = Mutex::new(Option::default());
        futures::stream::iter(start..=end)
            .map(|slot| async move {
                if slot % 100 == 0 {
                    info!("Update withdrawals current slot: {}", slot);
                }

                get_beacon_block_by_slot::<T>(&self.beacon, slot).await
            })
            .buffered(128)
            .for_each(|block| async {
                if block.is_none() {
                    return;
                }
                let block = block.unwrap();
                if let Err(err) = Self::insert_block_withdrawals(
                    tx.client(),
                    block.data.clone(),
                    &validator_indexes,
                )
                .await
                {
                    *err_ctx.lock().await = Some(err);
                }
            })
            .await;

        if let Some(err) = err_ctx.lock().await.as_ref() {
            error!("insert block withdrawals error: {}", err);
            return Err(anyhow!("{}", err.to_string()));
        }
        upsert_sync_state(
            tx.client(),
            &SyncState::WithdrawalFinalizedSlot,
            &(end as i64),
        )
        .await?;
        tx.commit().await?;
        Ok(())
    }
    async fn insert_block_withdrawals(
        client: &Client,
        block: SignedBeaconBlock<T>,
        validator_indexes: &HashSet<u64>,
    ) -> Result<()> {
        let block = block.message_capella().unwrap();
        let slot = block.slot.as_u64() as i64;
        let withdrawals = block
            .body
            .execution_payload
            .execution_payload
            .withdrawals
            .to_vec()
            .into_iter()
            .filter(|withdrawal| validator_indexes.contains(&withdrawal.validator_index))
            .collect();
        upsert_withdrawals(client, &withdrawals, slot).await?;
        Ok(())
    }
}

pub async fn get_beacon_block_by_slot<T: EthSpec>(
    client: &BeaconNodeHttpClient,
    slot: u64,
) -> Option<ExecutionOptimisticFinalizedForkVersionedResponse<SignedBeaconBlock<T>>> {
    loop {
        let result = client.get_beacon_blocks(BlockId::Slot(slot.into())).await;
        match result {
            Ok(data) => return data,
            Err(err) => {
                error!("get beacon blocks error: {}", err);
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_load_slots() {}
}
