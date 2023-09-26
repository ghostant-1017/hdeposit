use super::*;
use crate::beacon::{get_beacon_block_by_slot, get_validator_balances_by_slot, BeaconClient};
use anyhow::Context;
use backoff::{future::retry, ExponentialBackoff};
use eth2::types::EthSpec;
use futures::StreamExt;
use slot_clock::SlotClock;
use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
    sync::Arc,
};
use storage::models::{insert_protocol_rewards, ProtocolReward};
use storage::{
    db::PgPool,
    models::{select_all_validator_indexes, select_sync_state, upsert_sync_state, SyncState},
};
use tokio::sync::Mutex;
use tracing::*;

pub async fn sync_protocol_rewards<T: EthSpec>(
    pool: PgPool,
    eth: EthComponent,
) -> anyhow::Result<()> {
    let mut conn = pool.get().await?;
    let synced = select_sync_state(&conn, &SyncState::DailyRewardsEpoch)
        .await?
        .unwrap_or_default();
    let current = eth.clock.now().unwrap().epoch(T::slots_per_epoch());
    let start_epoch_of_today = current / 225 * 225;
    info!("Sync protocol rewards, synced epoch: {synced}, current epoch: {current}, start epoch of today: {start_epoch_of_today}");
    // We already have protocol rewards data in range: [synced, synced + 225)
    // And we only sync protocol rewards before yesterday
    if synced + 225 >= start_epoch_of_today.as_u64() {
        info!("Skip sync protocol rewards");
        return Ok(());
    }
    let validator_ids = select_all_validator_indexes(&conn)
        .await?;

    let beacon = eth.beacon.clone();
    let rewards = tokio::spawn(async move {
        get_protocol_rewards_daily::<T>(&beacon, synced + 225, &validator_ids).await
    })
    .await
    .context("join get protocol rewards daily")?
    .context("get protocol rewards daily")?;

    let tx = conn.transaction().await?;
    upsert_sync_state(
        tx.client(),
        &SyncState::DailyRewardsEpoch,
        &(synced as i64 + 225),
    )
    .await?;
    insert_protocol_rewards(tx.client(), &rewards).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn get_protocol_rewards_daily<T: EthSpec>(
    beacon: &BeaconClient,
    start_epoch_of_day: u64,
    validators_ids: &HashSet<u64>,
) -> anyhow::Result<Vec<ProtocolReward>> {
    let start_slot = start_epoch_of_day * T::slots_per_epoch();

    let end_epoch_of_day = start_epoch_of_day + 225;
    let end_slot = end_epoch_of_day * T::slots_per_epoch() - 1;
    info!("Extracting daily rewards start slot: {start_slot}, end_slot: {end_slot}");
    let start_balances = get_validator_balances_by_slot(beacon, start_slot, validators_ids).await?;
    let end_balances = get_validator_balances_by_slot(beacon, end_slot, validators_ids).await?;
    let withdrawals = Arc::new(Mutex::new(HashMap::<u64, u64>::new()));
    futures::stream::iter(start_slot..=end_slot)
        .map(|slot| async move {
            retry(ExponentialBackoff::default(), || async {
                Ok(get_beacon_block_by_slot::<T>(beacon, slot).await?)
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
            let data: Vec<_> = block
                .message_capella()
                .unwrap()
                .body
                .execution_payload
                .execution_payload
                .withdrawals
                .to_vec()
                .into_iter()
                .filter(|w| validators_ids.contains(&w.validator_index))
                .collect();
            for withdrawal in data {
                withdrawals
                    .lock()
                    .await
                    .entry(withdrawal.validator_index)
                    .or_default()
                    .add_assign(withdrawal.amount);
            }
        })
        .await;
    let withdrawals = Arc::try_unwrap(withdrawals).unwrap().into_inner();
    let mut result = vec![];
    for id in validators_ids {
        let start_balance = *start_balances.get(id).unwrap_or(&0);
        let closing_balance = *end_balances.get(id).unwrap_or(&0);
        let withdrawal_amount = *withdrawals.get(id).unwrap_or(&0);
        let reward_amount: i64;
        if start_balance == 0 && closing_balance == 0 {
            info!("Skip extract protocol reward validator: {}", id);
            continue;
        }
        if start_balance == 0 {
            reward_amount = withdrawal_amount as i64 + (closing_balance as i64 - 32_000_000_000);
        } else if closing_balance == 0 {
            reward_amount = withdrawal_amount as i64 - start_balance as i64;
        } else {
            reward_amount =
                closing_balance as i64 - start_balance as i64 + withdrawal_amount as i64;
        }
        result.push(ProtocolReward {
            epoch: start_epoch_of_day,
            validator_index: *id,
            start_balance,
            closing_balance,
            withdrawal_amount,
            reward_amount,
        })
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eth2::Timeouts;
    use slot_clock::{Slot, SlotClock};
    use std::time::Duration;
    fn sample_beacon_client() -> BeaconClient {
        let server = "https://stylish-soft-shadow.ethereum-goerli.discover.quiknode.pro/0ee6b1dcfb32c48a5ad26f4ff7157a26e1bc7537/";
        BeaconClient::new(
            server.parse().unwrap(),
            Timeouts::set_all(Duration::from_secs(5)),
        )
    }

    #[test]
    fn test_map() {
        let time = chrono::NaiveDateTime::from_timestamp_opt(1616508000, 0).unwrap();
        println!("GenesisTime: {}", time.and_utc());

        let clock = slot_clock::SystemTimeSlotClock::new(
            Slot::new(0),
            Duration::from_secs(1616508000),
            Duration::from_secs(12),
        );
        // let slot = clock.now().unwrap();
        // let epoch = slot.epoch(32);
        // let start_epoch_of_day = epoch / 225 * 225;
        // println!("Slot: {}", slot);
        // println!("Epoch: {}", epoch);
        // println!("EpochToday:{}", start_epoch_of_day);
        let now = chrono::Utc::now().timestamp();
        let now_slot = clock.slot_of(Duration::from_secs(now as u64)).unwrap();
        let now_epoch = now_slot.epoch(32);
        let start_epoch_of_today = now_epoch / 225 * 225;
        let end_epoch_of_today = start_epoch_of_today + 225;
        println!("start_slot_of_today: {}", start_epoch_of_today);
        println!("end_epoch_of_today: {}", end_epoch_of_today);
        // let now = chrono::Utc::now();

        // let today = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
        // println!("{}", today);
    }

    #[tokio::test]
    async fn test_extract_daily_rewards() {
        // let beacon = sample_beacon_client();
        // let mut validator_ids = HashSet::new();
        // validator_ids.insert(566889);
        // validator_ids.insert(509650);
        // validator_ids.insert(105778);
        // let result = get_protocol_rewards_daily::<MainnetEthSpec>(&beacon, 205650, &validator_ids)
        //     .await
        //     .unwrap();
        // println!("{:?}", result);
    }
}
