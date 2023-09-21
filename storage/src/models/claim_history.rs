use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;
use contract::elfee::SplitFeeFilter;
use ethers::prelude::LogMeta;
use ethers::types::Address;

pub async fn insert_claim(
    client: &Client,
    address: Address,
    log: SplitFeeFilter,
    meta: LogMeta,
) -> Result<()> {
    let sql = "insert into claim_history(el_fee_contract, log, meta) values($1, $2, $3);";
    let address = serde_json::to_string(&address)?;
    let log = serde_json::to_value(log)?;
    let meta = serde_json::to_value(meta)?;
    client.execute(sql, &[&address, &log, &meta]).await?;
    Ok(())
}

pub async fn select_claim_by_address(
    client: &Client,
    address: Address,
) -> Result<Vec<(SplitFeeFilter, LogMeta)>> {
    let sql = "select * from claim_history where meta->>'address' = $1";
    let rows = client
        .query(sql, &[&serde_json::to_string(&address)?])
        .await?;
    let mut result = vec![];
    for row in rows {
        let log: serde_json::Value = row.get("log");
        let meta: serde_json::Value = row.get("meta");
        result.push((serde_json::from_value(log)?, serde_json::from_value(meta)?));
    }
    Ok(result)
}
