use super::*;
use bb8_postgres::tokio_postgres::{Row, Client};
use ethers::prelude::LogMeta;

pub struct StoredPreDepositEvt {
    pub pk: i64,
    pub flattened: bool,
    pub log: PreDepositFilter,
    pub meta: LogMeta,
}

impl TryFrom<Row> for StoredPreDepositEvt {
    type Error = anyhow::Error;
    fn try_from(row: Row) -> std::result::Result<Self, Self::Error> {
        let pk = row.try_get("pk")?;
        let flattened = row.try_get("flattened")?;
        let pre_deposit_filter: serde_json::Value = row.try_get("pre_deposit_filter")?;
        let pre_deposit_filter = serde_json::from_value(pre_deposit_filter)?;
        let log_meta: serde_json::Value= row.try_get("log_meta")?;
        let log_meta = serde_json::from_value(log_meta)?;
        Ok(StoredPreDepositEvt { pk, flattened, log: pre_deposit_filter, meta: log_meta })
    }
}

pub async fn insert_batch_logs(
    conn: &mut PgConnection<'_>,
    logs: &Vec<(PreDepositFilter, LogMeta)>,
) -> Result<()> {
    let tx = conn.transaction().await?;
    for log in logs {
        let pre_deposit_filter = serde_json::to_value(&log.0)?;
        let log_meta = serde_json::to_value(&log.1)?;
        let block_number = log.1.block_number.as_u64() as i64;
        tx.execute("insert into pre_deposit_events (pre_deposit_filter, log_meta, block_number) values 
        ({}, {}, {});", &[&pre_deposit_filter, &log_meta, &block_number]).await?;
    }
    tx.commit().await?;
    Ok(())
}

pub async fn query_latest_block_number(conn: &mut PgConnection<'_>) -> Result<Option<u64>> {
    let row = conn
        .query_one("select max(block_number) from pre_deposit_events;", &[])
        .await?;
    let height: Option<i64> = row.get("max");
    Ok(height.map(|i| i as u64))
}

pub async fn query_unflattened_events(client: &Client) -> Result<Vec<StoredPreDepositEvt>> {
    let rows = client.query("select * from pre_deposit_events where flattened = false order by block_number;", &[]).await?;
    let mut result = vec![];
    for row in rows {
        let ks = row.try_into()?;
        result.push(ks);
    }
    Ok(result)
}