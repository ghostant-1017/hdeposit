use std::collections::HashSet;

use super::{*, withdrawals::get_beacon_block_by_slot};
use anyhow::{Result, anyhow};
use bb8_postgres::tokio_postgres::Client as PgClient;
use eth2::types::{ValidatorId, StateId, BlockId, SignedBeaconBlock};
use ethers::providers::Middleware;
use futures::StreamExt;
use storage::models::{select_sync_state, SyncState};
use tracing::info;

impl<T: EthSpec> Updater<T> {
    pub async fn update_el_fee(&self) -> Result<()> {
        let conn = self.pool.get().await?;
        // 1.Select last synced slot, let from = synced + 1;
        let synced = select_sync_state(&conn, &SyncState::ELFeeLastSlot)
        .await?
        .unwrap_or(self.start);
        let from = synced + 1;
        // 2.Query current finality slot, let to = finality;
        let finalized = self
            .beacon
            .get_beacon_blocks::<T>(BlockId::Finalized)
            .await
            .map_err(|err| anyhow!("{err}"))?
            .ok_or(anyhow!("get finalized unwrap"))?;
        let to = finalized.data.slot().as_u64();
        // 3.Get slots [from, to]
        futures::stream::iter(from..=to)
            .map(|slot| async move {
                if slot % 100 == 0 {
                    info!("Update el_fee current slot: {}", slot);
                }

                get_beacon_block_by_slot::<T>(&self.beacon, slot).await
            })
            .buffered(128)
            .for_each(|block| async {
                if block.is_none() {
                    return;
                }
                let block = block.unwrap().data;
                if let Err(err) = (
                )
                .await
                {
                    *err_ctx.lock().await = Some(err);
                }
            })
            .await;

        // 4. Iter the slot to check if the `proposer_index` belong to our validators
        
        // 5. Get related block_number in eth1, get the `receipt` and `fee`

        // 6. Insert the logs into db

        // 7. update last synced slot = to;

        Ok(())
    }

    pub async fn process_slot_el_fee(db: &PgClient, eth1: Provider<Http>, block: &SignedBeaconBlock<T>, validator_indexes: &HashSet<u64>) -> Result<()> {
        let message = block.message_capella().map_err(|_| anyhow!("not capella message"))?;
        let proposer_index = message.proposer_index;
        if !validator_indexes.contains(&proposer_index) {
            return Ok(())
        }
        let fee_recipient = message.body.execution_payload.execution_payload.fee_recipient;
        let block_number = message.body.execution_payload.execution_payload.block_number;
        let receipts = eth1.get_block_receipts(block_number).await?;
        
        todo!()
    }
}
