use anyhow::Result;
use bb8_postgres::tokio_postgres::{types::ToSql, Client};
pub enum SyncState {
    WithdrawalFinalizedSlot,
    WithdrawalLastSlot,
    DepositTxLastPK,
}

impl SyncState {
    pub fn to_key(&self) -> String {
        match self {
            SyncState::WithdrawalFinalizedSlot => "withdrawal_finalized_slot".to_string(),
            SyncState::WithdrawalLastSlot => "withdrawal_last_slot".to_string(),
            SyncState::DepositTxLastPK => "deposit_last_pk".to_string(),
        }
    }
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
