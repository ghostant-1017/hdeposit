use super::*;
use bb8_postgres::tokio_postgres::{Client, Row};
use eth2_keystore::Keystore;

pub struct StoredKeyStore {
    pub pk: i64,
    pub keystore: Keystore,
    pub deposit_data_pk: Option<i64>,
}

impl TryFrom<Row> for StoredKeyStore {
    type Error = anyhow::Error;

    fn try_from(row: Row) -> std::result::Result<Self, Self::Error> {
        let pk: i64 = row.try_get("pk")?;
        let data: serde_json::Value = row.try_get("keystore")?;
        let keystore =
            Keystore::from_json_str(&data.to_string()).map_err(|_| anyhow!("serde error"))?;
        let deposit_data_pk: Option<i64> = row.try_get("deposit_data_pk")?;
        Ok(StoredKeyStore {
            pk,
            keystore,
            deposit_data_pk,
        })
    }
}

pub async fn query_unused_keystore(client: &Client, n: i64) -> Result<Vec<StoredKeyStore>> {
    let rows = client
        .query(
            "select * from bls_keystore where deposit_data_pk is null limit $1;",
            &[&n],
        )
        .await?;
    let mut result = vec![];
    for row in rows {
        let ks = StoredKeyStore::try_from(row)?;
        result.push(ks);
    }
    Ok(result)
}

pub async fn update_keystore_fk(
    client: &Client,
    keystore: &StoredKeyStore,
    deposit_data_id: i64,
) -> Result<u64> {
    let result = client
        .execute(
            "update bls_keystore set deposit_data_pk = $1 where pk = $2;",
            &[&deposit_data_id, &keystore.pk],
        )
        .await?;
    Ok(result)
}
