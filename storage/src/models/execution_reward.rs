use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;
use ethers::types::{H160, H256};

use super::{
    select_max_epoch, select_range_active_validators_by_wc,
    select_wc_validator_indexes,
};

#[derive(Debug)]
pub struct ExecutionReward {
    pub slot: u64,
    pub block_number: u64,
    pub block_hash: H256,
    pub validator_index: u64,
    pub fee_recipient: H160,
    pub amount: u64,
}

pub async fn insert_execution_reward(client: &Client, batch: &Vec<ExecutionReward>) -> Result<()> {
    let sql =
        "insert into execution_reward(slot,block_number,block_hash,validator_index,fee_recipient,amount)
    values($1,$2,$3,$4,$5,$6);";
    for el_fee in batch {
        client
            .execute(
                sql,
                &[
                    &(el_fee.slot as i64),
                    &(el_fee.block_number as i64),
                    &(serde_json::to_string(&el_fee.block_hash)?),
                    &(el_fee.validator_index as i64),
                    &(serde_json::to_string(&el_fee.fee_recipient)?),
                    &(el_fee.amount as i64),
                ],
            )
            .await?;
    }
    Ok(())
}

pub async fn select_validator_cumulative_el_reward(
    client: &Client,
    validator_index: u64,
) -> Result<u64> {
    let sql = "select (sum(amount) / 1000000000)::Bigint as cumulative_reward from execution_reward where validator_index = $1;";
    let row = client.query_one(sql, &[&(validator_index as i64)]).await?;
    let data: Option<i64> = row.get("cumulative_reward");
    Ok(data.map(|n| n as u64).unwrap_or_default())
}

pub async fn select_wc_el_apr_7d(client: &Client, wc: H256) -> Result<f64> {
    let indexes: Vec<i64> = select_wc_validator_indexes(client, wc)
        .await?
        .into_iter()
        .map(|i| i as i64)
        .collect();
    let end_epoch = select_max_epoch(client).await? as i64;
    let start_epoch = end_epoch - 6 * 225;

    // Caculate the total balance as the denominator
    let validator_count =
        select_range_active_validators_by_wc(client, start_epoch, end_epoch, wc).await?;
    let total = select_range_el_fee_by_indexes(client, start_epoch * 32, end_epoch * 32, &indexes)
        .await? as f64;
    let apr = total / (validator_count as f64) / 32_000_000_000.0 / 7.0 * 365.0 * 100.0;

    Ok(apr)
}

pub async fn select_range_el_fee_by_indexes(
    client: &Client,
    from: i64,
    to: i64,
    indexes: &Vec<i64>,
) -> Result<i64> {
    let sql = "
    select (sum(amount) / 1000000000)::BIGINT from execution_reward where validator_index = any($1) 
        and 
    slot >= $2 
        and 
    slot <= $3;";
    let row = client.query_one(sql, &[&indexes, &from, &to]).await?;
    let reward: Option<i64> = row.get(0);
    Ok(reward.unwrap_or_default())
}
