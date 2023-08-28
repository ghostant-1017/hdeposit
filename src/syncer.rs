use std::sync::Arc;
use std::time::Duration;

use crate::db::PgPool;
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

pub struct EventService {
    contract: Vault<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pool: PgPool,
}

impl EventService {
    pub fn new(endpoint: &str, contract_addr: Address, wallet: LocalWallet, pool: PgPool) -> Result<Self> {
        let provider = ethers::providers::Provider::try_from(endpoint)?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = Vault::new(contract_addr, client);
        Ok(Self { contract, pool })
    }

    pub fn start_update_service(self) -> Result<()> {
        tokio::spawn(async move {
            let mut from = self.fetch_last_synced().await.unwrap();
            info!("Syncing eth1 events from: {}", from);
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
        let conn = self.pool.get().await?;
        todo!()
    }

    async fn fetch_last_finality(&self) -> Result<u64> {
        todo!()
    }

    async fn insert_batch(&self, logs: Vec<(PreDepositFilter, LogMeta)>) -> Result<()> {
        todo!()
    }

    async fn do_update(&self, from: u64) -> Result<u64> {
        let to = self.fetch_last_finality().await?;
        ensure!(from <= to, "Critical bug or Ethereum finality broken");
        let logs = self.query_pre_deposit_logs(from, to).await?;
        self.insert_batch(logs).await.context("insert batch")?;
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
