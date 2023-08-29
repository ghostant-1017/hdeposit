use super::*;
use anyhow::ensure;
use bb8_postgres::tokio_postgres::{Row, Client};
use eth2_keystore::Keystore;

pub struct StoredKeyStore {
    pub pk: i64,
    pub key_store: Keystore,
    pub deposit_data_pk: Option<i64>
}

impl TryFrom<Row> for StoredKeyStore {
    type Error = anyhow::Error;

    fn try_from(row: Row) -> std::result::Result<Self, Self::Error> {
        let pk: i64 = row.try_get("pk")?;
        let data: serde_json::Value = row.try_get("key_store")?;
        let key_store = Keystore::from_json_str(&data.to_string()).map_err(|_| anyhow!("serde error"))?;
        let deposit_data_pk: Option<i64> = row.try_get("deposit_data_pk")?;
        Ok(StoredKeyStore { pk, key_store, deposit_data_pk })
    }
}

pub async fn query_unused_key_store(client: &Client, n: i64) -> Result<Vec<StoredKeyStore>> {
    let rows = client
        .query("select * from bls_keystore where deposit_data_pk is null limit $1;", &[&n])
        .await?;
    let mut result = vec![];
    for row in rows {
        let ks = StoredKeyStore::try_from(row)?;
        result.push(ks);
    }
    ensure!(result.len() == n as usize, "Not enough bls keystore, expect: {}, found: {}.", n, result.len());
    return Ok(result)
}

pub async fn update_key_store_fk(client: &Client, key_store: &StoredKeyStore, deposit_data_id: i64) -> Result<()> {
    let result = client.execute("update bls_keystore set deposit_data_pk = {} where pk = {};", &[&deposit_data_id, &key_store.pk]).await?;
    ensure!(result == 1, "update bls_keystore fail");
    return Ok(())
}