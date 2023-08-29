use super::*;
use anyhow::ensure;
use bb8_postgres::tokio_postgres::Row;
use eth2_keystore::Keystore;

pub struct StoredKeyStore {
    pub pk: i64,
    pub key_store: Keystore,
    pub event_pk: Option<i64>
}

impl TryFrom<Row> for StoredKeyStore {
    type Error = anyhow::Error;

    fn try_from(row: Row) -> std::result::Result<Self, Self::Error> {
        let pk: i64 = row.try_get("pk")?;
        let data: serde_json::Value = row.try_get("key_store")?;
        let key_store = Keystore::from_json_str(&data.to_string()).map_err(|_| anyhow!("serde error"))?;
        let event_pk: Option<i64> = row.try_get("event_pk")?;
        Ok(StoredKeyStore { pk, key_store, event_pk })
    }
}

pub async fn query_unused_key_store(conn: &mut PgConnection<'_>, n: i64) -> Result<Vec<StoredKeyStore>> {
    let rows = conn
        .query("select * from bls_addresses where event_pk is null limit {};", &[&n])
        .await?;
    let mut result = vec![];
    for row in rows {
        let ks = StoredKeyStore::try_from(row)?;
        result.push(ks);
    }
    ensure!(result.len() == n as usize, "Not enough bls keystore, expect: {}, found: {}.", n, result.len());
    return Ok(result)
}