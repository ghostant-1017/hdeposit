use std::collections::HashSet;

use super::*;
use anyhow::{anyhow, Result};
use bb8_postgres::tokio_postgres::Client;
use eth2::types::{BlockId, SignedBeaconBlock};
use storage::models::{
    select_all_validators, select_sync_state, upsert_sync_state, upsert_withdrawals, SyncState,
};
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
        for slot in start..=end {
            let block = self
                .beacon
                .get_beacon_blocks(BlockId::Slot(slot.into()))
                .await
                .map_err(|err| anyhow!("{err}"))?
                .unwrap()
                .data;
            Self::insert_block_withdrawals(tx.client(), block, &validator_indexes).await?;
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
        let withdrawals = block
            .message_capella()
            .unwrap()
            .body
            .execution_payload
            .execution_payload
            .withdrawals
            .to_vec()
            .into_iter()
            .filter(|withdrawal| validator_indexes.contains(&withdrawal.validator_index))
            .collect();
        upsert_withdrawals(client, &withdrawals).await?;
        Ok(())
    }
}
