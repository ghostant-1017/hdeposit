use std::collections::HashMap;
use std::collections::HashSet;

use std::sync::Arc;

use crate::beacon::get_beacon_block_by_slot;
use crate::beacon::get_current_finalized_block;

use crate::component::EthComponent;
use crate::geth::get_block_receipts_by_number;
use crate::geth::get_block_transactions_by_number;
use crate::geth::Eth1Client;
use anyhow::anyhow;
use anyhow::ensure;
use anyhow::Context;
use backoff::future::retry;
use backoff::ExponentialBackoff;

use eth2::types::EthSpec;
use eth2::types::SignedBeaconBlock;
use eth2::types::Slot;
use ethers::types::Block;
use ethers::types::Transaction;
use ethers::types::TransactionReceipt;
use ethers::types::H256;
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
    })
    .await
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
    let amount = caculate_block_fee(eth1, block_number).await?;
    Ok(ExecutionReward {
        slot,
        block_number,
        block_hash: block_hash.into_root(),
        validator_index,
        fee_recipient,
        amount,
    })
}

async fn caculate_block_fee(client: &Eth1Client, block_number: u64) -> anyhow::Result<u64> {
    let block = get_block_transactions_by_number(client, block_number)
        .await?
        .ok_or(anyhow!("block transactions empty"))?;
    let receipts = get_block_receipts_by_number(client, block_number).await?;

    let mut total = 0;
    let base_fee_per_gas = block
        .base_fee_per_gas
        .ok_or(anyhow!("base_fee_per_gas not found"))?;
    for receipt in receipts {
        let effective_gas_price = receipt
            .effective_gas_price
            .ok_or(anyhow!("effective_gas_price not found"))?;
        let gas_price = effective_gas_price - base_fee_per_gas;

        let gas_used = receipt.gas_used.ok_or(anyhow!("gas_used not found"))?;
        total += gas_price.as_u64() * gas_used.as_u64();
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::tasks::el_reward::caculate_block_fee;

    #[tokio::test]
    async fn test_caculate_block_fee() {
        // Write test for function `caculate_block_fee`'
        let eth1 = "https://stylish-soft-shadow.ethereum-goerli.discover.quiknode.pro/0ee6b1dcfb32c48a5ad26f4ff7157a26e1bc7537/";
        let eth1 = Arc::new(ethers::providers::Provider::try_from(eth1).unwrap());
        let fee = caculate_block_fee(&eth1, 9825330).await.unwrap();
        println!("fee: {:?}", fee);
    }
}
