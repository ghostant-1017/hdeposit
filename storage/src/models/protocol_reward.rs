use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;
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
    let sql = "select max(epoch)::Bigint as max_epoch from protocol_reward;";
    let row = client.query_one(sql, &[]).await?;
    let max_epoch: Option<i64> = row.get("max_epoch");
    if max_epoch.is_none() {
        return Ok(Default::default())
    }
    let max_epoch = max_epoch.unwrap();
    let start_epoch = max_epoch - 6 * 225;

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