use anyhow::{anyhow, Result};
use bb8_postgres::tokio_postgres::Client;
use contract::deposit::DepositEventFilter;
use eth2::types::ValidatorData;
use ethers::types::U64;
use lighthouse_types::{Hash256, PublicKey};

pub async fn upsert_validators(client: &Client, validators: &Vec<ValidatorData>) -> Result<()> {
    let sql = "insert into hellman_validators(index,pubkey,withdrawal_credentials,amount,data) 
    values($1, $2, $3, $4,$5) 
    on conflict (index) do update set pubkey=$2,withdrawal_credentials=$3,amount=$4,data=$5;";
    for validator in validators {
        // TODO: Optimize sql
        let index = validator.index as i64;
        let pubkey = serde_json::to_string(&validator.validator.pubkey)?;
        let wc = serde_json::to_string(&validator.validator.withdrawal_credentials)?;
        let amount = validator.balance as i64;
        let data = serde_json::to_value(validator)?;
        client
            .execute(sql, &[&index, &pubkey, &wc, &amount, &data])
            .await?;
    }
    Ok(())
}

pub async fn select_validators_by_credentials(
    client: &Client,
    wc: Hash256,
) -> Result<Vec<ValidatorData>> {
    let sql = "select * from hellman_validators where withdrawal_credentials = $1";
    let rows = client.query(sql, &[&serde_json::to_string(&wc)?]).await?;
    let mut result = vec![];
    for row in rows {
        let data = serde_json::from_value(row.get("data"))?;
        result.push(data);
    }
    Ok(result)
}

pub async fn select_all_validators(client: &Client) -> Result<Vec<ValidatorData>> {
    let sql = "select * from hellman_validators;";
    let rows = client.query(sql, &[]).await?;
    let mut result = vec![];
    for row in rows {
        let data = serde_json::from_value(row.get("data"))?;
        result.push(data);
    }
    Ok(result)
}

pub async fn upsert_validators_by_logs(
    client: &Client,
    logs: &Vec<DepositEventFilter>,
) -> Result<()> {
    let sql = "insert into hellman_validators(index,pubkey,withdrawal_credentials,amount) 
    values($1, $2, $3, $4) 
    on conflict (index) do nothing";
    for log in logs {
        let index = U64::from_little_endian(&log.index).as_u64() as i64;
        let pubey = PublicKey::deserialize(&log.pubkey).map_err(|err| anyhow!("{err:?}"))?;
        let wc = Hash256::from_slice(&log.withdrawal_credentials);
        let amount = U64::from_little_endian(&log.amount).as_u64() as i64;
        client
            .execute(
                sql,
                &[
                    &index,
                    &serde_json::to_string(&pubey)?,
                    &serde_json::to_string(&wc)?,
                    &amount,
                ],
            )
            .await?;
    }
    Ok(())
}
