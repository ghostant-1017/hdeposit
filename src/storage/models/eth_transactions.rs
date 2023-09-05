use anyhow::{anyhow, Result};
use bb8_postgres::tokio_postgres::{Client, Row};
use ethers::types::{transaction::eip2718::TypedTransaction, Bytes as EBytes, Signature};
use lighthouse_types::Hash256;

pub struct StoredEthTransaction {
    pub pk: i64,
    pub tx: TypedTransaction,
    pub tx_hash: Hash256,
    pub signature: Signature,
    pub finality: bool,
}

impl TryFrom<Row> for StoredEthTransaction {
    type Error = anyhow::Error;

    fn try_from(row: Row) -> std::result::Result<Self, Self::Error> {
        let pk: i64 = row.try_get("pk")?;
        let tx_hash: String = row.try_get("tx_hash")?;
        let tx_hash = serde_json::from_str(&tx_hash)?;
        let tx: serde_json::Value = row.try_get("tx")?;
        let tx: TypedTransaction = serde_json::from_value(tx)?;
        let signature: String = row.try_get("signature")?;
        let signature = serde_json::from_str(&signature)?;
        let finality = row.try_get("finality")?;
        Ok(StoredEthTransaction {
            pk,
            tx,
            tx_hash,
            signature,
            finality,
        })
    }
}

impl StoredEthTransaction {
    pub fn raw_tx(&self) -> EBytes {
        self.tx.rlp_signed(&self.signature)
    }
}

pub async fn insert_eth_transaction(
    client: &Client,
    tx: TypedTransaction,
    signature: Signature,
) -> Result<i64> {
    let tx_hash = tx.hash(&signature);
    let mut serde_tx = serde_json::to_value(&tx)?;
    serde_tx["chainId"] = serde_json::to_value(tx.chain_id().ok_or(anyhow!("Missing chaiId"))?)?;
    let result = client
        .query_one(
            "insert into eth_transactions (tx_hash, tx, signature) values ($1, $2, $3) returning pk;",
            &[&serde_json::to_string(&tx_hash)?, &serde_tx, &serde_json::to_string(&signature)?],
        )
        .await?;
    let pk = result.try_get("pk")?;
    Ok(pk)
}

pub async fn select_pending_eth_transactions(client: &Client) -> Result<Vec<StoredEthTransaction>> {
    let rows = client
        .query(
            "select * from eth_transactions where finality = false;",
            &[],
        )
        .await?;
    let mut result = vec![];
    for row in rows {
        let tx = row.try_into()?;
        result.push(tx)
    }
    Ok(result)
}

pub async fn update_eth_tx_to_finality(client: &Client, tx_hash: Hash256) -> Result<u64> {
    let result = client
        .execute(
            "update eth_transactions set finality = true where tx_hash = $1",
            &[&serde_json::to_string(&tx_hash)?],
        )
        .await?;
    Ok(result)
}
