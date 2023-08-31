use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;
use ethers::types::{transaction::eip2718::TypedTransaction, Signature};
use lighthouse_types::Hash256;

pub struct StoredEthTransaction {
    pk: i64,
    tx: TypedTransaction,
    tx_hash: Hash256,
    signature: Signature,
}

pub async fn insert_eth_transaction(
    client: &Client,
    tx: TypedTransaction,
    signature: Signature,
) -> Result<i64> {
    let tx_hash = tx.hash(&signature).to_string();
    let tx = serde_json::to_value(tx)?;
    let result = client
        .query_one(
            "insert into eth_transactions (tx_hash, tx, signature) values ($1, $2, $3) returning pk;",
            &[&tx_hash, &tx, &signature.to_string()],
        )
        .await?;
    let pk = result.try_get("pk")?;
    Ok(pk)
}
