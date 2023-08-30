use anyhow::Result;
use bb8_postgres::tokio_postgres::{Client, Row};
use lighthouse_types::DepositData;

use super::{StoredKeyStore, StoredPreDepositEvt};

pub struct StoredDepositData {
    pub pk: i64,
    pub deposit_data: DepositData,
    pub evt_pk: i64,
}

impl TryFrom<Row> for StoredDepositData {
    type Error = anyhow::Error;
    fn try_from(row: Row) -> std::result::Result<Self, Self::Error> {
        let pk: i64 = row.try_get("pk")?;
        let deposit_data: serde_json::Value = row.try_get("data")?;
        let evt_pk = row.try_get("pre_deposit_event_pk")?;
        let deposit_data = serde_json::from_value(deposit_data)?;
        Ok(StoredDepositData { pk, deposit_data, evt_pk })
    }
}

pub async fn insert_deposit_data(
    client: &Client,
    evt: &StoredPreDepositEvt,
    deposit_data: &DepositData,
    _key_store: &StoredKeyStore,
) -> Result<i64> {
    let data = serde_json::to_value(deposit_data)?;
    let pre_deposit_event_pk = evt.pk;
    let row = client
        .query_one(
            "insert into deposit_data (pre_deposit_event_pk, data) values
            ($1, $2) returning pk;",
            &[
                &pre_deposit_event_pk,
                &data,
            ],
        )
        .await?;
    let id: i64 = row.try_get("pk")?;
    Ok(id)
}

pub async fn query_pending_deposit_data(client: &Client) -> Result<Vec<StoredDepositData>>{
    let rows = client.query("select * from deposit_data order by pk ASC;", &[]).await?;
    let mut result = vec![];
    for row in rows {
        result.push(row.try_into()?)
    }
    Ok(result)
}