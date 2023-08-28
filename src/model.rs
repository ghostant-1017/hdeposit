use ethers::prelude::LogMeta;
use crate::{vault::PreDepositFilter, db::PgConnection};
use anyhow::Result;

pub async fn insert_batch_logs(conn: &mut PgConnection<'_>, logs: &Vec<(PreDepositFilter, LogMeta)>) -> Result<()> {
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
    Ok(height.and_then(|i| Some(i as u64)))
}