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
        let address = serde_json::from_str(row.try_get("address")?)?;
        let block_number = serde_json::from_str(row.try_get("block_number")?)?;
        let block_hash = serde_json::from_str(row.try_get("block_hash")?)?;
        let transaction_hash = serde_json::from_str(row.try_get("transaction_hash")?)?;
        let transaction_index = serde_json::from_str(row.try_get("transaction_index")?)?;
        let log_index = serde_json::from_str(row.try_get("log_index")?)?;
        let sender = serde_json::from_str(row.try_get("sender")?)?;
        let n = serde_json::from_str(row.try_get("n")?)?;
        let create_el_fee = row.try_get("create_el_fee")?;
        let withdrawal_credential = serde_json::from_str(row.try_get("withdrawal_credential")?)?;
        let el_fee_contract = serde_json::from_str(row.try_get("el_fee_contract")?)?;
        let log = PreDepositFilter {
            sender,
            n,
            create_el_fee,
            withdrawal_credential,
            el_fee_contract,
        };
        let meta = LogMeta {
            address,
            block_number,
            block_hash,
            transaction_hash,
            transaction_index,
            log_index,
        };
        Ok(StoredPreDepositEvt { pk, flattened, log, meta })
    }
}

pub async fn insert_batch_logs(
    conn: &mut PgConnection<'_>,
    logs: &Vec<(PreDepositFilter, LogMeta)>,
) -> Result<()> {
    let tx = conn.transaction().await?;
    for log in logs {
        let address = log.1.address;
        let block_number = log.1.block_number;
        let block_hash = log.1.block_hash;
        let transaction_hash = log.1.transaction_hash;
        let transaction_index = log.1.transaction_index;
        let log_index = log.1.log_index;
        let sender = log.0.sender;
        let n = log.0.n;
        let create_el_fee = log.0.create_el_fee;
        let withdrawal_credential = log.0.withdrawal_credential.clone();
        let el_fee_contract = log.0.el_fee_contract;
        let sql = format!("insert into pre_deposit_events 
        (address, block_number, block_hash, transaction_hash, transaction_index, log_index, sender, n, create_el_fee, withdrawal_credential, el_fee_contract) values 
        ('{address}', '{block_number}', '{block_hash}', '{transaction_hash}','{transaction_index}', '{log_index}', '{sender}', '{n}','{create_el_fee}', '{withdrawal_credential}', '{el_fee_contract}');");
        tx.execute(&sql, &[]).await?;
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