use super::*;

use bb8_postgres::tokio_postgres::{Client, Row};
use ethers::{prelude::LogMeta, types::Address};
use lighthouse_types::Hash256;

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
        let log_meta: serde_json::Value = row.try_get("log_meta")?;
        let log_meta = serde_json::from_value(log_meta)?;
        Ok(StoredPreDepositEvt {
            pk,
            flattened,
            log: pre_deposit_filter,
            meta: log_meta,
        })
    }
}

pub async fn insert_batch_logs(
    client: &Client,
    logs: &Vec<(PreDepositFilter, LogMeta)>,
) -> Result<()> {
    for log in logs {
        let pre_deposit_filter = serde_json::to_value(&log.0)?;
        let log_meta = serde_json::to_value(&log.1)?;
        let block_number = log.1.block_number.as_u64() as i64;
        client.execute(
            "insert into pre_deposit_events (pre_deposit_filter, log_meta, block_number) values 
        ($1, $2, $3);",
            &[&pre_deposit_filter, &log_meta, &block_number],
        )
        .await?;
    }
    Ok(())
}

pub async fn query_latest_block_number(client: &Client) -> Result<Option<u64>> {
    let row = client
        .query_one("select max(block_number) from pre_deposit_events;", &[])
        .await?;
    let height: Option<i64> = row.get("max");
    Ok(height.map(|i| i as u64))
}

pub async fn query_unflattened_events(client: &Client) -> Result<Vec<StoredPreDepositEvt>> {
    let rows = client
        .query(
            "select * from pre_deposit_events where flattened = false order by block_number;",
            &[],
        )
        .await?;
    let mut result = vec![];
    for row in rows {
        let ks = row.try_into()?;
        result.push(ks);
    }
    Ok(result)
}

pub async fn query_all_events(client: &Client) -> Result<Vec<StoredPreDepositEvt>> {
    let rows = client
        .query("select * from pre_deposit_events;", &[])
        .await?;
    let mut result = vec![];
    for row in rows {
        let evt = row.try_into()?;
        result.push(evt);
    }
    Ok(result)
}

pub async fn update_events_to_flattened(client: &Client, pk: i64) -> Result<u64> {
    let result = client
        .execute(
            "update pre_deposit_events set flattened = true where pk=$1",
            &[&pk],
        )
        .await?;
    Ok(result)
}

pub async fn query_el_fee_address_by_wc(client: &Client, wc: &Hash256) -> Result<Option<Address>> {
    let sql = "select * from pre_deposit_events where pre_deposit_filter->>'withdrawal_credential' = $1 and pre_deposit_filter->>'create_el_fee' = 'true' limit 1;";
    let wc = serde_json::to_string(wc)?;
    let wc = wc.trim_matches('"');
    let result = client.query_opt(sql, &[&wc]).await?;
    let stored_evt: StoredPreDepositEvt = match result {
        Some(row) => row.try_into()?,
        None => return Ok(None),
    };
    Ok(Some(stored_evt.log.el_fee_contract))
}

pub async fn query_all_el_fee_contract(client: &Client) -> Result<Vec<Address>> {
    let sql = "SELECT DISTINCT pre_deposit_filter->>'el_fee_contract' from pre_deposit_events;";
    let rows = client.query(sql, &[]).await?;
    let mut result = vec![];
    for row in rows {
        let address: String = row.get(0);
        let address = format!("\"{}\"", address);
        result.push(serde_json::from_str(&address)?);
    }
    Ok(result)
}

pub async fn query_contract_deployed_block_number(
    client: &Client,
    address: Address,
) -> Result<Option<u64>> {
    let sql = "select * from pre_deposit_events where pre_deposit_filter->>'el_fee_contract' = $1 order by pk asc limit 1;";
    let address = serde_json::to_string(&address)?;
    let address = address.trim_matches('"');
    let row = client.query_opt(sql, &[&address]).await?;
    let row = match row {
        Some(row) => row,
        None => return Ok(None),
    };
    let evt: StoredPreDepositEvt = row.try_into()?;
    Ok(Some(evt.meta.block_number.as_u64()))
}
