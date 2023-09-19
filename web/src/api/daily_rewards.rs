use std::time::Duration;

use anyhow::anyhow;
use chrono::{Days, NaiveDateTime};
use eth2::types::Hash256;
use indexmap::IndexMap;
use slot_clock::Slot;
use storage::models::select_withdrawals_by_wc_range;

use super::*;

const SLOTS_PER_DAY: u64 = 7200;

#[derive(Debug, Deserialize)]
pub struct Params {
    wc: Hash256,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub data: IndexMap<NaiveDateTime, u64>,
}

pub async fn get_daily_rewards_7days(
    Query(params): Query<Params>,
    State(server): State<Server>,
) -> Result<Json<Response>, AppError> {
    let wc = params.wc;
    let db = server.pool.get().await?;
    let now = chrono::Utc::now();
    let today = now
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .ok_or(anyhow!("time error"))?;
    // Caculate the range [start, end)
    let end = server
        .clock
        .slot_of(Duration::from_secs(today.and_utc().timestamp() as u64))
        .ok_or(anyhow!("slot error"))?;
    let start = end - SLOTS_PER_DAY * 7;
    let batch =
        select_withdrawals_by_wc_range(&db, wc, start.as_u64() as i64, end.as_u64() as i64).await?;

    let clock = server.clock;
    // 1. Initialize HashMap: Date -> amount
    let mut result = IndexMap::<NaiveDateTime, u64>::new();
    for i in 1..=7 {
        let day = today
            .checked_sub_days(Days::new(i))
            .ok_or(anyhow!("sub error"))?;
        result.insert(day, 0);
    }
    // 2. Iter batch to sum amount
    for w in batch {
        let day = slot_to_day(&clock, w.slot)?;
        result
            .entry(day)
            .and_modify(|val| *val += w.withdrawal.amount);
    }
    let response = Response { data: result };
    Ok(Json(response))
}

fn slot_to_day(clock: &SystemTimeSlotClock, slot: Slot) -> anyhow::Result<NaiveDateTime> {
    let d = clock.start_of(slot).ok_or(anyhow!("start of slot"))?;
    let day = NaiveDateTime::from_timestamp_opt(d.as_secs() as i64, 0)
        .ok_or(anyhow!("from timestamp opt"))?;
    let day = day
        .date()
        .and_hms_opt(0, 0, 0)
        .ok_or(anyhow!("and hms opt"))?;
    Ok(day)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_map() {
        let now = chrono::Utc::now();
        let today = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let mut result = IndexMap::<NaiveDateTime, u64>::new();
        for i in 1..=7 {
            let day = today
                .checked_sub_days(Days::new(i))
                .ok_or(anyhow!("sub error"))
                .unwrap();
            result.insert(day, 0);
        }
        println!("{:?}", result);
    }
}
