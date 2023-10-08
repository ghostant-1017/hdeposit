use std::collections::HashSet;

use std::sync::Arc;

use crate::beacon::get_beacon_block_by_slot;
use crate::beacon::get_current_finalized_block;

use crate::component::EthComponent;
use crate::geth::get_block_receipts_by_hash;
use crate::geth::Eth1Client;
use anyhow::Context;
use anyhow::anyhow;
use anyhow::ensure;
use backoff::future::retry;
use backoff::ExponentialBackoff;

use eth2::types::EthSpec;
use eth2::types::SignedBeaconBlock;
use eth2::types::Slot;
use ethers::types::TransactionReceipt;
use futures::StreamExt;
use storage::db::PgPool;
use storage::models::insert_execution_reward;
use storage::models::select_all_validator_indexes;
use storage::models::select_sync_state;
use storage::models::upsert_sync_state;
use storage::models::ExecutionReward;
use storage::models::SyncState;
use tokio::sync::Mutex;
use tracing::info;

pub async fn sync_execution_rewards<T: EthSpec>(
    pool: PgPool,
    eth: EthComponent,
) -> anyhow::Result<()> {
    // Check if we have synced to the latest finalized state
    let mut db = pool.get().await?;
    let synced = Slot::new(
        select_sync_state(&db, &SyncState::ELRewardLastSlot)
            .await?
            .unwrap(),
    );
    let finalized = get_current_finalized_block::<T>(&eth.beacon).await?.slot();
    ensure!(
        synced <= finalized,
        "Critical error, synced must less than finalized slot"
    );
    if finalized == synced {
        return Ok(());
    }

    // Now we have new slots to sync with, range: [synced + 1, finalized_slot]
    info!(
        "Syncing execution rewards from: {} to: {}",
        synced + 1,
        finalized
    );
    let validator_ids = select_all_validator_indexes(&db).await?;
    let rewards = tokio::spawn(async move {
        get_execution_rewards::<T>(&eth, &validator_ids, synced + 1, finalized).await
    }).await
    .context("join get execution rewards")?
    .context("get execution rewards")?;
    
    // Insert batch in transacitons and update SyncState
    let tx = db.transaction().await?;
    insert_execution_reward(tx.client(), &rewards).await?;
    upsert_sync_state(
        tx.client(),
        &SyncState::ELRewardLastSlot,
        &(finalized.as_u64() as i64),
    )
    .await?;
    tx.commit().await?;

    info!(
        "Successfully synced execution rewards nums: {}",
        rewards.len()
    );
    Ok(())
}

pub async fn get_execution_rewards<T: EthSpec>(
    eth: &EthComponent,
    validator_ids: &HashSet<u64>,
    from: Slot,
    to: Slot,
) -> anyhow::Result<Vec<ExecutionReward>> {
    let from = from.as_u64();
    let to = to.as_u64();
    let rewards = Arc::new(Mutex::new(vec![]));
    futures::stream::iter(from..=to)
        .map(|slot| async move {
            retry(ExponentialBackoff::default(), || async {
                Ok(get_beacon_block_by_slot::<T>(&eth.beacon, slot).await?)
            })
            .await
            .unwrap()
        })
        .buffered(128)
        .for_each(|block| async {
            let block = match block {
                Some(block) => block,
                None => return,
            };
            // If the proposer is not our validators, just skip
            let proposer_index = block.message_capella().unwrap().proposer_index;
            if !validator_ids.contains(&proposer_index) {
                return;
            }
            // Extract execution reward inforamtion from beacon block
            let reward = retry(ExponentialBackoff::default(), || async {
                Ok(extract_el_rewards_capella::<T>(&block, &eth.eth1).await?)
            })
            .await
            .unwrap();
            rewards.lock().await.push(reward);
        })
        .await;
    let rewards = Arc::try_unwrap(rewards).unwrap().into_inner();
    Ok(rewards)
}

pub async fn extract_el_rewards_capella<T: EthSpec>(
    block: &SignedBeaconBlock<T>,
    eth1: &Eth1Client,
) -> anyhow::Result<ExecutionReward> {
    let block = &block
        .as_capella()
        .map_err(|_| anyhow!("not capella block"))?
        .message;
    let slot = block.slot.as_u64();
    let validator_index = block.proposer_index;

    let fee_recipient = block.body.execution_payload.execution_payload.fee_recipient;
    let block_number = block.body.execution_payload.execution_payload.block_number;
    let block_hash = block.body.execution_payload.execution_payload.block_hash;
    // 3. query block_hash from eth1
    let receipts = get_block_receipts_by_hash(eth1, block_number).await?;
    let amount = caculate_block_fee(receipts)?;
    Ok(ExecutionReward {
        slot,
        block_number,
        block_hash: block_hash.into_root(),
        validator_index,
        fee_recipient,
        amount,
    })
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
