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
        (epoch, validator_index, start_balance, closing_balance, withdrawl_amount, reward_amount)
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
