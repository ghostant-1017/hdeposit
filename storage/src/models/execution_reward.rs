use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;
use ethers::types::{H160, H256};

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
