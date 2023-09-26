use super::*;
use anyhow::anyhow;

use bb8_postgres::tokio_postgres::Client;
use eth2::types::Hash256;

use slot_clock::Slot;
use storage::models::{select_wc_validator_indexes, select_sync_state, SyncState};

const SLOTS_PER_DAY: u64 = 7200;

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

#[derive(Debug, Serialize)]
pub struct WalletDailyReward {
    unix: u64,
    epoch: i64,
    withdrawal: i64,
    protocol_reward: i64,
    cumulative_protocol_reward: i64,
    closing_balance: i64,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub total_items: i64,
    pub data: Vec<WalletDailyReward>,
}

pub async fn get_daily_rewards_7days(
    Path(params): Path<Params>,
    State(server): State<Server>,
) -> Result<Json<Response>, AppError> {
    let wc = params.wc;
    let db = server.pool.get().await?;
    let clock = server.clock;
    let data = get_recent_n_days_rewards(&db, &clock, 15, wc).await?;
    let total_items = data.len() as i64;
    Ok(Json(Response { total_items, data }))
}

async fn get_recent_n_days_rewards(db: &Client, clock: &SystemTimeSlotClock, n: u64, wc: Hash256) -> anyhow::Result<Vec<WalletDailyReward>> {
    let end_epoch = select_sync_state(&db, &SyncState::DailyRewardsEpoch)
    .await?
    .ok_or(anyhow!("missing protocol rewards"))? + 225;
    let start_epoch = end_epoch - 225 * n;
    let validator_ids: Vec<i64> = select_wc_validator_indexes(&db, wc)
        .await?
        .into_iter()
        .map(|id| id as i64)
        .collect();

    let sql = "select 
        sum(reward_amount)::bigint as cumulative_protocol_reward
    from 
        protocol_reward
    where 
        validator_index = any($1)
    and 
        epoch < $2;";
    let row = db
        .query_one(sql, &[&validator_ids, &(start_epoch as i64)])
        .await?;
    let cumulative_protocol_reward: Option<i64> = row.get("cumulative_protocol_reward");
    let mut cumulative_protocol_reward = cumulative_protocol_reward.unwrap_or_default();

    let sql = "select epoch, 
    sum(reward_amount)::bigint as reward,
    sum(withdrawal_amount)::bigint as withdrawal,
    sum(closing_balance)::bigint as closing_balance
        from protocol_reward
    where validator_index = any($1)
    and 
        epoch >= $2
    GROUP BY epoch 
    ORDER BY epoch;";
    let mut data = vec![];
    let rows = db
        .query(sql, &[&validator_ids, &(start_epoch as i64)])
        .await?;
    for row in rows {
        let epoch: i64 = row.get("epoch");
        let protocol_reward: i64 = row.get("reward");
        let withdrawal: i64 = row.get("withdrawal");
        let closing_balance: i64 = row.get("closing_balance");
        cumulative_protocol_reward += protocol_reward;
        data.push(WalletDailyReward {
            unix: epoch_to_timestamp(&clock, epoch as u64)?,
            epoch,
            protocol_reward,
            withdrawal,
            closing_balance,
            cumulative_protocol_reward,
        })
    }
    Ok(data)
}

pub fn epoch_to_timestamp(clock: &SystemTimeSlotClock, epoch: u64) -> anyhow::Result<u64> {
    // TODO: replace `slots_per_epoch` of
    let slot = Slot::new(epoch * 32);
    let time = clock.start_of(slot).ok_or(anyhow!("start of slot error"))?;
    Ok(time.as_secs())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_map() {}
}
