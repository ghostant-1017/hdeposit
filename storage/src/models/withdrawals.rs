use anyhow::Result;
use bb8_postgres::tokio_postgres::{Client, Row};
use lighthouse_types::{Withdrawal, Address};


pub async fn upsert_withdrawals(
    client: &Client,
    withdrawals: &Vec<Withdrawal>,
) -> Result<()> {
    for withdrawal in withdrawals {
        client
            .execute(
                "insert into withdrawals (index, validator_index, address, amount) values ($1, $2, $3, $4)
                    on conflict (index) do update 
                set validator_index = $2, address = $3, amount = $4;",
                &[
                    &(withdrawal.index as i64),
                    &(withdrawal.validator_index as i64),
                    &(serde_json::to_string(&withdrawal.address))?,
                    &(withdrawal.amount as i64)
                ],
            )
            .await?;
    }
    Ok(())
}

pub async fn select_withdrawals_by_validator_index(client: &Client, validator_index: u64) -> Result<Vec<Withdrawal>> {
    let rows = client
        .query(
            "select * from withdrawals where validator_index = $1;",
            &[&(validator_index as i64)],
        )
        .await?;
    let mut results = vec![];
    for row in rows {
        let withdrawal = row_to_withdrawal(row)?;
        results.push(withdrawal);
    }
    Ok(results)
}

fn row_to_withdrawal(row: Row) -> Result<Withdrawal> {
    let index: i64 = row.get("index");
    let validator_index: i64 = row.get("validator_index");
    let address: String = row.get("address");
    let address: Address = serde_json::from_str(&address)?;
    let amount: i64 = row.get("amount");
    Ok(Withdrawal { index: index as u64, validator_index: validator_index as u64, address, amount: amount as u64 })
}

pub async fn select_last_slot(client: &Client) -> Result<u64> {
    let row = client
        .query_opt("select max(slot) from withdrawals;", &[])
        .await?;
    let slot: i64 = match row {
        Some(row) => row.get(0),
        None => 0,
    };
    Ok(slot as u64)
}
