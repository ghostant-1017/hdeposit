use std::sync::Arc;
use std::time::Duration;

use crate::db::PgPool;
use crate::eth2::get_current_finality;
use crate::model::{insert_batch_logs, query_latest_height};
use crate::vault::{PreDepositFilter, Vault};
use anyhow::{ensure, Context, Result};
use ethers::prelude::LogMeta;
use ethers::prelude::SignerMiddleware;
use ethers::{
    providers::{Http, Provider},
    signers::{LocalWallet, Wallet},
    types::Address,
};
use k256::ecdsa::SigningKey;
use tokio::time::sleep;
use tracing::*;
use url::Url;

#[derive(Clone)]
pub struct EventService {
    contract: Vault<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pool: PgPool,
    eth2_base: Url,
}

impl EventService {
    pub fn new(eth1_base: Url, eth2_base: Url, contract_addr: Address, wallet: LocalWallet, pool: PgPool, ) -> Result<Self> {
        let provider = ethers::providers::Provider::try_from(eth1_base.as_str())?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = Vault::new(contract_addr, client);
        Ok(Self { contract, pool, eth2_base })
    }

    pub async fn start_update_service(self) -> Result<()> {
        let mut from = self.fetch_last_synced().await?;
        tokio::spawn(async move {
            info!("Start syncing eth1 events from: {}", from);
            loop {
                match self.do_update(from).await {
                    Ok(synced) => {
                        info!("Sync events sucess, from {from} to {synced}");
                        from = synced;
                    }
                    Err(err) => {
                        error!("Eth1 sync error: {:#}", err);
                    },
                };
                sleep(Duration::from_secs(12)).await;
            }
        });
        Ok(())
    }

    async fn fetch_last_synced(&self) -> Result<u64> {
        let mut conn = self.pool.get().await?;
        let height = query_latest_height(&mut conn).await?;
        Ok(height)
    }

    async fn get_current_finality(&self) -> Result<u64> {
        get_current_finality(&self.eth2_base).await
    }

    async fn insert_batch(&self, logs: Vec<(PreDepositFilter, LogMeta)>) -> Result<()> {
        let mut conn = self.pool.get().await?;
        insert_batch_logs(&mut conn, &logs).await?;
        Ok(())
    }

    async fn do_update(&self, from: u64) -> Result<u64> {
        let to = self.get_current_finality().await.context("get current finality")?;
        ensure!(from <= to, "Critical bug or Ethereum finality broken");
        debug!("Querying logs from {from} to {to}");
        let logs = self.query_pre_deposit_logs(from, to).await?;
        self.insert_batch(logs).await?;
        debug!("Insert logs from {from} to {to} success");
        Ok(to)
    }

    pub async fn query_pre_deposit_logs(
        &self,
        from: u64,
        to: u64,
    ) -> Result<Vec<(PreDepositFilter, LogMeta)>> {
        let logs = self
            .contract
            .pre_deposit_filter()
            .from_block(from)
            .to_block(to)
            .query_with_meta()
            .await?;
        Ok(logs)
    }
}
