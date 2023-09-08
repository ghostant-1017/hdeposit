use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;
use eth2::types::ValidatorData;
use lighthouse_types::Hash256;

pub async fn insert_or_update_validators(
    client: &Client,
    validators: &Vec<ValidatorData>,
) -> Result<()> {
    let sql = "insert into hellman_validators(index, withdrawal_credentials, data)
    values ($1, $2, $3) 
    on conflict (index) do update set data = $3;";
    for validator in validators {
        // TODO: Optimize sql
        let index = validator.index as i64;
        let wc = serde_json::to_string(&validator.validator.withdrawal_credentials)?;
        let data = serde_json::to_value(validator)?;
        client.execute(sql, &[&index, &wc, &data]).await?;
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
