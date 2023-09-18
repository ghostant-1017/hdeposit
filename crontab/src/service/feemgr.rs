use std::time::Duration;

use anyhow::Result;
use eth2::lighthouse_vc::{
    http_client::ValidatorClientHttpClient, types::UpdateFeeRecipientRequest,
};
use storage::{
    db::PgPool,
    models::{query_all_events, query_deposit_data_by_evt_pk},
};
use tracing::{error, info, warn};

pub struct FeeManagerService {
    pub pool: PgPool,
    pub validator_client: ValidatorClientHttpClient,
}

impl FeeManagerService {
    pub fn new(validator_client: ValidatorClientHttpClient, pool: PgPool) -> Self {
        Self {
            validator_client,
            pool,
        }
    }

    pub fn start_update_service(self) -> Result<()> {
        tokio::spawn(async move {
            loop {
                if let Err(err) = self.do_update().await {
                    error!("[FeeManager] do update error: {err}");
                }
                tokio::time::sleep(Duration::from_secs(60 * 60)).await
            }
        });
        Ok(())
    }

    async fn do_update(&self) -> Result<()> {
        // 1.Query `pre_deposit_events` from db and related `DepositData`
        let conn = self.pool.get().await?;
        let evts = query_all_events(&conn).await?;
        for evt in evts {
            if !evt.log.create_el_fee {
                warn!(
                    "[FeeManager]Found `PreDepositEvent` create_el_fee is false, pk: {}.",
                    evt.pk
                );
                continue;
            };
            let pubkeys: Vec<_> = query_deposit_data_by_evt_pk(&conn, evt.pk)
                .await?
                .into_iter()
                .map(|data| data.deposit_data.pubkey)
                .collect();
            for pubkey in pubkeys {
                let req = UpdateFeeRecipientRequest {
                    ethaddress: evt.log.el_fee_contract,
                };
                info!(
                    "[FeeManager]Trying to set pubkey: {}, fee recipient: {}",
                    pubkey, req.ethaddress
                );
                if let Err(err) = self
                    .validator_client
                    .post_fee_recipient(&pubkey, &req)
                    .await
                {
                    error!("[FeeManager]Set fee recipient error: {err}");
                }
            }
        }
        Ok(())
    }
}
