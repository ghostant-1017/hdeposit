use anyhow::Result;
use bb8_postgres::tokio_postgres::{Client, Row};
use lighthouse_types::{Address, Hash256, Slot, Withdrawal};

pub struct StoredWithdrawal {
    pub slot: Slot,
    pub withdrawal: Withdrawal,
}

pub async fn upsert_withdrawals(
    client: &Client,
    withdrawals: &Vec<Withdrawal>,
    slot: i64,
) -> Result<()> {
    for withdrawal in withdrawals {
        client
            .execute(
                "insert into withdrawals (index, validator_index, address, amount, slot) values ($1, $2, $3, $4, $5)
                    on conflict (index) do update 
                set validator_index = $2, address = $3, amount = $4, slot = $5;",
                &[
                    &(withdrawal.index as i64),
                    &(withdrawal.validator_index as i64),
                    &(serde_json::to_string(&withdrawal.address))?,
                    &(withdrawal.amount as i64),
                    &slot
                ],
            )
            .await?;
    }
    Ok(())
}

pub async fn select_withdrawals_by_validator_index(
    client: &Client,
    validator_index: u64,
) -> Result<Vec<StoredWithdrawal>> {
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

pub async fn select_withdrawals_by_wc_range(
    client: &Client,
    wc: Hash256,
    start: i64,
    end: i64,
) -> Result<Vec<StoredWithdrawal>> {
    let sql = "select * from withdrawals where slot >= $1 and slot < $2 and address = $3;";
    let address = serde_json::to_string(&wc_to_address(wc))?;
    let rows = client.query(sql, &[&start, &end, &address]).await?;
    let mut results = vec![];
    for row in rows {
        let withdrawal = row_to_withdrawal(row)?;
        results.push(withdrawal);
    }
    Ok(results)
}

fn wc_to_address(wc: Hash256) -> Address {
    let (_, address) = wc.as_bytes().split_at(12);
    Address::from_slice(address)
}

fn row_to_withdrawal(row: Row) -> Result<StoredWithdrawal> {
    let index: i64 = row.get("index");
    let validator_index: i64 = row.get("validator_index");
    let address: String = row.get("address");
    let address: Address = serde_json::from_str(&address)?;
    let amount: i64 = row.get("amount");
    let slot: i64 = row.get("slot");
    let withdrawal = Withdrawal {
        index: index as u64,
        validator_index: validator_index as u64,
        address,
        amount: amount as u64,
    };
    let stored_withdrawal = StoredWithdrawal {
        withdrawal,
        slot: (slot as u64).into(),
    };
    Ok(stored_withdrawal)
}
