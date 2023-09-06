use anyhow::{anyhow, Result};
use eth2::types::{StateId, ValidatorId};
use storage::models::{insert_or_update_validators, query_used_keystore};

use super::Updater;

impl Updater {
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
            .get_beacon_states_validators(StateId::Finalized, Some(&validator_ids), None)
            .await
            .map_err(|err| anyhow!("{err}"))?
            .unwrap();
        let validators = validators.data;
        insert_or_update_validators(&conn, &validators).await?;
        Ok(())
    }
}
