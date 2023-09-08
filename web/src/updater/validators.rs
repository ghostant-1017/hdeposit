use anyhow::{anyhow, Result};
use eth2::types::{StateId, ValidatorId, EthSpec};
use storage::models::{insert_or_update_validators, query_used_keystore};
use tracing::info;

use super::Updater;

impl<T: EthSpec> Updater<T>  {
    pub async fn update_validators(&self) -> Result<()> {
        let conn = self.pool.get().await?;
        let validator_ids: Vec<_> = query_used_keystore(&conn)
            .await?
            .into_iter()
            .map(|ks| ks.keystore.public_key().unwrap())
            .map(|pk| ValidatorId::PublicKey(pk.into()))
            .collect();
        let validators = self
            .beacon
            .get_beacon_states_validators(StateId::Head, Some(&validator_ids), None)
            .await
            .map_err(|err| anyhow!("{err}"))?
            .unwrap();
        let validators = validators.data;
        info!("Total: {}, Prepare to update validators: {}", validator_ids.len(), validators.len());
        insert_or_update_validators(&conn, &validators).await?;
        Ok(())
    }
}
