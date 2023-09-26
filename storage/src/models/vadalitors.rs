use std::collections::HashSet;

use anyhow::{anyhow, Result};
use bb8_postgres::tokio_postgres::{Client, Row};
use contract::deposit::DepositEventFilter;
use eth2::types::ValidatorData;
use ethers::types::U64;
use lighthouse_types::{Hash256, PublicKey};

pub struct HellmanValidator {
    pub index: Option<u64>,
    pub pubkey: PublicKey,
    pub withdrawal_credentials: Hash256,
    pub amount: u64,
    pub data: Option<ValidatorData>,
}

impl TryFrom<Row> for HellmanValidator {
    type Error = anyhow::Error;

    fn try_from(row: Row) -> Result<Self> {
        let index: Option<i64> = row.get("index");
        let pubkey: String = row.get("pubkey");
        let wc: String = row.get("withdrawal_credentials");
        let amount: i64 = row.get("amount");
        let data: Option<serde_json::Value> = row.get("data");
        let validator_data = match data {
            Some(data) => Some(serde_json::from_value(data)?),
            None => None,
        };
        Ok(Self {
            index: index.map(|index| index as u64),
            pubkey: serde_json::from_str(&pubkey)?,
            withdrawal_credentials: serde_json::from_str(&wc)?,
            amount: amount as u64,
            data: validator_data,
        })
    }
}

pub async fn upsert_validators(client: &Client, validators: &Vec<ValidatorData>) -> Result<()> {
    let sql = "insert into hellman_validators(index,pubkey,withdrawal_credentials,amount,data) 
    values($1, $2, $3, $4,$5) 
    on conflict (pubkey) do update set index=$1,pubkey=$2,withdrawal_credentials=$3,amount=$4,data=$5;";
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
) -> Result<Vec<HellmanValidator>> {
    let sql = "select * from hellman_validators where withdrawal_credentials = $1";
    let rows = client.query(sql, &[&serde_json::to_string(&wc)?]).await?;
    let mut result = vec![];
    for row in rows {
        result.push(row.try_into()?);
    }
    Ok(result)
}

pub async fn select_all_validators(client: &Client) -> Result<Vec<HellmanValidator>> {
    let sql = "select * from hellman_validators;";
    let rows = client.query(sql, &[]).await?;
    let mut result = vec![];
    for row in rows {
        result.push(row.try_into()?);
    }
    Ok(result)
}

pub async fn select_all_validator_indexes(client: &Client) -> Result<HashSet<u64>> {
    let sql = "select index from hellman_validators where index is not null;";
    let rows = client.query(sql, &[]).await?;
    let mut result = HashSet::new();
    for row in rows {
        let index: i64 = row.get("index");
        result.insert(index as u64);
    }
    Ok(result)
}

pub async fn select_wc_validator_indexes(client: &Client, wc: Hash256) -> Result<HashSet<u64>> {
    let sql = "select index from hellman_validators where index is not null and  withdrawal_credentials = $1;";
    let rows = client.query(sql, &[&serde_json::to_string(&wc)?]).await?;
    let mut result = HashSet::new();
    for row in rows {
        let index: i64 = row.get("index");
        result.insert(index as u64);
    }
    Ok(result)
}

pub async fn upsert_validators_by_logs(
    client: &Client,
    logs: &Vec<DepositEventFilter>,
) -> Result<()> {
    let sql = "insert into hellman_validators(pubkey,withdrawal_credentials,amount) 
    values($1, $2, $3) 
    on conflict (pubkey) do nothing";
    for log in logs {
        let pubey = PublicKey::deserialize(&log.pubkey).map_err(|err| anyhow!("{err:?}"))?;
        let wc = Hash256::from_slice(&log.withdrawal_credentials);
        let amount = U64::from_little_endian(&log.amount).as_u64() as i64;
        client
            .execute(
                sql,
                &[
                    &serde_json::to_string(&pubey)?,
                    &serde_json::to_string(&wc)?,
                    &amount,
                ],
            )
            .await?;
    }
    Ok(())
}

pub async fn select_validator_by_index(
    client: &Client,
    validator_index: u64,
) -> Result<Option<HellmanValidator>> {
    let sql = "select * from hellman_validators where index = $1";
    let result = client.query_opt(sql, &[&(validator_index as i64)]).await?;
    match result {
        Some(row) => Ok(Some(row.try_into()?)),
        None => Ok(None),
    }
}
