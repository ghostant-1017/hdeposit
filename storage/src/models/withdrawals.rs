use bb8_postgres::tokio_postgres::{Client, Row};
use lighthouse_types::Withdrawal;
use anyhow::Result;

// pub struct StoredWithdrawal {
//     pub index: u64, // pk
//     pub slot: u64,
//     pub data: Withdrawal
// }

// impl TryFrom<Row> for StoredWithdrawal {
//     type Error = anyhow::Error;

//     fn try_from(value: Row) -> std::result::Result<Self, Self::Error> {
//         todo!()
//     }
// }

pub async fn insert_withdrawals(client: &Client,slot: u64, withdrawals: &Vec<Withdrawal>) -> Result<()> {
    for withdrawal in withdrawals {
        client.execute("insert into withdrawals (index, slot, data) values ();",
         &[&(withdrawal.index as i64), &(slot as i64), &serde_json::to_value(&withdrawal)?]).await?;
    }
    Ok(())
}

pub async fn select_withdrawals(client: &Client, validator_index: u64) -> Result<Vec<Withdrawal>> {
    let rows = client.query("select data from withdrawals where validator_index = $1;", &[&(validator_index as i64)]).await?;
    let mut results = vec![];
    for row in rows {
        let data = row.get("data");
        let withdrawal = serde_json::from_value(data)?;
        results.push(withdrawal);
    }
    return Ok(results)
}

pub async fn select_last_slot(client: &Client) -> Result<u64> {
    let row = client.query_opt("select max(slot) from withdrawals;", &[]).await?;
    let slot: i64= match row {
        Some(row) => row.get(0),
        None => 0,
    };
    return Ok(slot as u64)
}