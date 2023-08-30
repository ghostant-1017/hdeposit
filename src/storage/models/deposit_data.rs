use anyhow::Result;
use bb8_postgres::tokio_postgres::Client;
use ethers::types::Bytes;
use lighthouse_types::DepositData;
use tree_hash::TreeHash;

use super::{StoredKeyStore, StoredPreDepositEvt};

pub struct StoredDepositData {
    pub pk: i64,
    pub deposit_data: DepositData,
    pub evt_pk: i64,
}

pub async fn insert_deposit_data(
    client: &Client,
    evt: &StoredPreDepositEvt,
    deposit_data: &DepositData,
    _key_store: &StoredKeyStore,
) -> Result<i64> {
    let deposit_data_root = deposit_data.tree_hash_root();
    let withdrawal_credential = evt.log.withdrawal_credential.clone();
    let pre_deposit_event_pk = evt.pk;
    let signature = deposit_data.signature.clone();
    let row = client
        .query_one(
            "insert into deposit_data 
    (pre_deposit_event_pk, signature, deposit_data_root, withdrawal_credential)
    values
    ($1, $2, $3, $4) returning pk;",
            &[
                &pre_deposit_event_pk,
                &signature.to_string(),
                &deposit_data_root.to_string(),
                &withdrawal_credential.to_string(),
            ],
        )
        .await?;
    let id: i64 = row.try_get("pk")?;
    Ok(id)
}
