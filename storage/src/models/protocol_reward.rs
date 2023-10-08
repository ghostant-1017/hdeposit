use std::collections::HashMap;

use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;
use ethers::types::H256;
use lighthouse_types::Epoch;

use super::select_wc_validator_indexes;
#[derive(Debug)]
pub struct ProtocolReward {
    pub epoch: u64,
    pub validator_index: u64,
    pub start_balance: u64,
    pub closing_balance: u64,
    pub withdrawal_amount: u64,
    pub reward_amount: i64,
}

pub async fn insert_protocol_rewards(client: &Client, batch: &Vec<ProtocolReward>) -> Result<()> {
    for reward in batch {
        client
            .execute(
                "insert into protocol_reward
        (epoch, validator_index, start_balance, closing_balance, withdrawal_amount, reward_amount)
        values
        ($1, $2, $3, $4, $5, $6);
        ",
                &[
                    &(reward.epoch as i64),
                    &(reward.validator_index as i64),
                    &(reward.start_balance as i64),
                    &(reward.closing_balance as i64),
                    &(reward.withdrawal_amount as i64),
                    &reward.reward_amount,
                ],
            )
            .await?;
    }
    Ok(())
}

pub async fn select_validator_cumulative_cl_reward(client: &Client, validator_index: u64) -> Result<u64> {
    let sql = "select sum(reward_amount)::Bigint as cumulative_reward from protocol_reward where validator_index = $1;";
    let row = client.query_one(sql, &[&(validator_index as i64)]).await?;
    let data: Option<i64> = row.get("cumulative_reward");
    Ok(data.map(|n| n as u64).unwrap_or_default())
}

pub async fn select_validator_cl_apr_7d(client: &Client, validator_index: u64) -> Result<f64> {
    let max_epoch = select_max_epoch(client).await?;
    let start_epoch = (max_epoch - 6 * 225) as i64;
    let sql = "select (sum(reward_amount) / 32000000000 / 7 * 365 * 100)::DOUBLE PRECISION as apr_7d
    from 
        protocol_reward 
    where 
        epoch >= $2
    and 
        validator_index = $1;";
    
    let row = client.query_one(sql, &[&(validator_index as i64), &start_epoch]).await?;
    let apr: Option<f64> = row.get("apr_7d");
    Ok(apr.unwrap_or_default())
}

pub async fn select_wc_cl_apr_7d(client: &Client, wc: H256) -> Result<f64> {
    let indexes: Vec<i64> = select_wc_validator_indexes(client, wc).await?
    .into_iter()
    .map(|i| i as i64)
    .collect();
    let start_epoch = select_max_epoch(client).await? - 6 * 225;
    let sql = "
select avg(t1.apr_7d)::DOUBLE PRECISION from
    (
        select validator_index, (sum(reward_amount) / 32000000000 / 7 * 365 * 100)::DOUBLE PRECISION as apr_7d, count(epoch)
        from 
            protocol_reward 
        where 
            validator_index = any($1)
        and
            epoch >= $2
        GROUP BY validator_index
    ) t1
where t1.count > 1;
";

    let row = client.query_one(sql, &[&indexes, &(start_epoch as i64)]).await?;
    let apr_7d: Option<f64> = row.get("avg");
    Ok(apr_7d.unwrap_or_default())
} 

pub async fn select_max_epoch(client: &Client) -> Result<u64> {
    let sql = "select max(epoch)::Bigint as max_epoch from protocol_reward;";
    let row = client.query_one(sql, &[]).await?;
    let max_epoch: Option<i64> = row.get("max_epoch");

    Ok(max_epoch.map(|n| n as u64).unwrap_or_default())
}

pub async fn select_range_validators_count(client: &Client, from: i64, to: i64) -> Result<Vec<(Epoch, u64)>> {
    let sql = "select epoch, count(validator_index)::BIGINT from protocol_reward WHERE 
        epoch >= $1 
    and 
        epoch <= $2
    and 
        reward_amount != 0 
    GROUP BY epoch
    ORDER BY epoch;";
    let rows = client.query(sql, &[&from, &to]).await?;
    let mut result = vec![];
    for row in rows {
        let epoch: i64 = row.get("epoch");
        let count: i64 = row.get("count");
        result.push((Epoch::new(epoch as u64), count as u64))
    }
    return Ok(result)
}

pub async fn select_range_cl_rewards(client: &Client, from: i64, to: i64) -> Result<Vec<(Epoch, u64)>> {
    let sql = "select epoch, sum(reward_amount)::BIGINT as cl_reward 
    from 
        protocol_reward 
    where 
        epoch >= $1
    and 
        epoch <= $2
    GROUP BY epoch 
    ORDER BY epoch ASC;";
    let rows = client.query(sql, &[&from, &to]).await?;
    let mut result = vec![];
    for row in rows {
        let epoch: i64 = row.get("epoch");
        let cl_reward: i64 = row.get("cl_reward");
        result.push((Epoch::new(epoch as u64), cl_reward as u64))
    }
    Ok(result)
}

// Epoch range
pub async fn select_range_el_rewards(client: &Client, from: i64, to: i64) -> Result<Vec<(Epoch, u64)>> {
    let sql = "
    select (slot / 32 / 225 * 225)::Bigint as epoch, sum(amount)::BIGINT as el_reward 
    from execution_reward 
    where 
        (slot / 32) >= $1
    and 
        (slot / 32) <= $2
    GROUP BY (slot / 32 / 225 * 225);";
    let rows = client.query(sql, &[&from, &to]).await?;
    let mut result = vec![];
    for row in rows {
        let epoch: i64 = row.get("epoch");
        let el_reward: i64 = row.get("el_reward");
        result.push((Epoch::new(epoch as u64), el_reward as u64))
    }
    Ok(result)
}