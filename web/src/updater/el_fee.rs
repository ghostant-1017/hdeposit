use std::collections::HashSet;

use super::{withdrawals::get_beacon_block_by_slot, *};
use anyhow::{anyhow, Result};
use bb8_postgres::tokio_postgres::Client as PgClient;
use eth2::types::{BlockId, SignedBeaconBlock};
use ethers::{providers::Middleware, types::TransactionReceipt};
use futures::StreamExt;
use storage::models::{
    insert_el_fee, select_all_validator_indexes, select_sync_state, upsert_sync_state,
    ExecutionReward, SyncState,
};
use tokio::sync::Mutex;
use tracing::info;

impl<T: EthSpec> Updater<T> {
    pub async fn update_el_fee(&self) -> Result<()> {
        let mut conn = self.pool.get().await?;
        // 1.Select last synced slot, let from = synced + 1;
        let tx = conn.transaction().await?;
        let synced = select_sync_state(tx.client(), &SyncState::ELRewardLastSlot)
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
        if synced == to {
            return Ok(());
        }
        // 3.Get slots [from, to]
        let err_ctx = Mutex::new(Option::default());
        let validator_indexes = select_all_validator_indexes(tx.client()).await?;
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
                if let Err(err) = Self::process_slot_el_fee(
                    tx.client(),
                    &self.eth1_client,
                    &block,
                    &validator_indexes,
                )
                .await
                {
                    *err_ctx.lock().await = Some(err);
                }
            })
            .await;
        upsert_sync_state(tx.client(), &SyncState::ELRewardLastSlot, &(to as i64)).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn process_slot_el_fee(
        db: &PgClient,
        eth1: &Provider<Http>,
        block: &SignedBeaconBlock<T>,
        validator_indexes: &HashSet<u64>,
    ) -> Result<()> {
        let message = block
            .message_capella()
            .map_err(|_| anyhow!("not capella message"))?;
        let proposer_index = message.proposer_index;
        if !validator_indexes.contains(&proposer_index) {
            return Ok(());
        }
        let slot = message.slot.as_u64();
        let fee_recipient = message
            .body
            .execution_payload
            .execution_payload
            .fee_recipient;
        let block_number = message
            .body
            .execution_payload
            .execution_payload
            .block_number;
        let block_hash = message
            .body
            .execution_payload
            .execution_payload
            .block_hash
            .into_root();
        let receipts = eth1.get_block_receipts(block_number).await?;
        let amount = caculate_block_fee(receipts)?;
        info!(
            "Found 32stake propose a slot, index: {proposer_index}, fee_recipient: {fee_recipient}"
        );
        let el_fee = ExecutionReward {
            slot,
            block_number,
            block_hash,
            validator_index: proposer_index,
            fee_recipient,
            amount,
        };
        insert_el_fee(db, el_fee).await?;
        Ok(())
    }
}

fn caculate_block_fee(receipts: Vec<TransactionReceipt>) -> anyhow::Result<u64> {
    let mut total = 0;
    for receipt in receipts {
        let gas_price = receipt
            .effective_gas_price
            .ok_or(anyhow!("effective_gas_price not found"))?
            .as_u64();
        let gas_used = receipt
            .gas_used
            .ok_or(anyhow!("gas_used not found"))?
            .as_u64();
        total += gas_price * gas_used;
    }
    Ok(total)
}
