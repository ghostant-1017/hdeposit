use super::*;
use eth2_keystore::Keystore;

pub async fn query_unused_key_store(conn: &mut PgConnection<'_>) -> Result<Keystore> {
    let row = conn
        .query_one("select * from bls_addresses limit 1;", &[])
        .await?;
    let data: serde_json::Value = row.get("key_store");
    // let ks: JsonKeystore= serde_json::from_str(&data)?;
    Keystore::from_json_str(&data.to_string()).map_err(|_| anyhow!("serde error"))
}