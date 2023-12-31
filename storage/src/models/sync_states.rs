use anyhow::Result;
use bb8_postgres::tokio_postgres::{types::ToSql, Client};
use ethers::types::Address;
pub enum SyncState {
    DepositTxLastPK,
    ContractLogs(Address),
    DailyRewardsEpoch,
    ELRewardLastSlot,
}

impl SyncState {
    pub fn to_key(&self) -> String {
        match self {
            SyncState::DepositTxLastPK => "deposit_last_pk".to_string(),
            SyncState::ContractLogs(address) => {
                format!("contract_logs_{}", serde_json::to_string(&address).unwrap())
            }
            SyncState::DailyRewardsEpoch => "daily_rewards_epoch".to_string(),
            SyncState::ELRewardLastSlot => "el_reward_last_slot".to_string(),
        }
    }
}

pub async fn init_sync_states(client: &Client, start_slot: i64) -> Result<()> {
    let sql = "insert into sync_states(name, val) values($1, $2) on conflict(name) do nothing;";
    client.execute(sql, &[&SyncState::ELRewardLastSlot.to_key(), &start_slot]).await?;
    client.execute(sql, &[&SyncState::DailyRewardsEpoch.to_key(),&(start_slot / 32 / 225 * 225)]).await?;
    Ok(())
}

pub async fn select_sync_state(client: &Client, state: &SyncState) -> Result<Option<u64>> {
    let value = client
        .query_opt(
            "select val from sync_states where name = $1;",
            &[&state.to_key()],
        )
        .await?;
    match value {
        Some(row) => {
            let value: i64 = row.get("val");
            Ok(Some(value as u64))
        }
        None => Ok(None),
    }
}

pub async fn upsert_sync_state(
    client: &Client,
    state: &SyncState,
    val: &(dyn ToSql + Sync),
) -> Result<()> {
    client
        .execute(
            "insert into sync_states (name, val) values ($1, $2) 
            on conflict (name)
            do update set val = $2;",
            &[&state.to_key(), val],
        )
        .await?;
    Ok(())
}
