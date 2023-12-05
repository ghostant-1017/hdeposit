use eth2::types::{ValidatorId, StateId};
use storage::models::{query_used_keystore, upsert_validators};

use super::*;

pub async fn sync_validator_info<T: EthSpec>(pool: PgPool,eth: EthComponent) -> anyhow::Result<()> {
    let conn = pool.get().await?;
    let validator_ids: Vec<_> = query_used_keystore(&conn)
        .await?
        .into_iter()
        .map(|ks| ks.keystore.public_key().unwrap())
        .map(|pk| ValidatorId::PublicKey(pk.into()))
        .collect();
    let validators = eth
        .beacon
        .get_beacon_states_validators(StateId::Head, Some(&validator_ids), None)
        .await
        .map_err(|err| anyhow!("{err}"))?
        .unwrap();
    let validators = validators.data;
    tracing::info!(
        "Total: {}, Prepare to update validators: {}",
        validator_ids.len(),
        validators.len()
    );
    upsert_validators(&conn, &validators).await?;
    Ok(())
}