use std::marker::PhantomData;
use std::time::Duration;

use crate::eth2::get_current_finality_block_number;
use crate::storage::db::PgPool;
use crate::storage::models::{insert_batch_logs, query_latest_block_number};
use crate::vault::PreDepositFilter;
use anyhow::{anyhow, ensure, Context, Result};
use eth2::BeaconNodeHttpClient;
use ethers::prelude::LogMeta;

use lighthouse_types::EthSpec;
use tokio::time::sleep;
use tracing::*;

use super::VaultContract;

#[derive(Clone)]
pub struct EventService<T: EthSpec> {
    contract: VaultContract,
    pool: PgPool,
    eth2_client: BeaconNodeHttpClient,
    _p: PhantomData<T>,
}

impl<T: EthSpec> EventService<T> {
    pub fn new(
        eth2_client: BeaconNodeHttpClient,
        contract: VaultContract,
        pool: PgPool,
    ) -> Result<Self> {
        Ok(Self {
            contract,
            pool,
            eth2_client,
            _p: Default::default(),
        })
    }

    pub async fn start_update_service(self, start: u64) -> Result<()> {
        let synced = self
            .fetch_last_synced()
            .await
            .context("fetch last synced")?;
        let mut from = synced.unwrap_or(start).saturating_add(1);
        // tokio::spawn(async move {
        info!("[Syncer]Start syncing eth1 events from: {}", from);
        loop {
            match self.do_update(from).await {
                Ok(synced) => {
                    from = synced;
                }
                Err(err) => {
                    error!("[Syncer]Eth1 sync error: {:#}", err);
                }
            };
            sleep(Duration::from_secs(12)).await;
        }
        // });
        // Ok(())
    }

    async fn do_update(&self, from: u64) -> Result<u64> {
        let to = self
            .get_current_finality_block_number()
            .await
            .context("get current finality")?;
        info!("[Syncer]Current finality block number: {to}");
        ensure!(
            from - 1 <= to,
            "Critical bug or Ethereum finality broken, synced: {}, finality: {}",
            from,
            to
        );
        if from - 1 == to {
            return Ok(from);
        }
        info!("[Syncer]Querying logs from {from} to {to}");
        let logs = self.query_pre_deposit_logs(from, to).await?;
        self.insert_batch(&logs).await?;
        info!(
            "[Syncer]Insert logs from {from} to {to} success, nums: {}",
            logs.len()
        );
        Ok(to + 1)
    }
}

// DB trait
impl<T: EthSpec> EventService<T> {
    async fn fetch_last_synced(&self) -> Result<Option<u64>> {
        let conn = self.pool.get().await?;
        let height = query_latest_block_number(&conn).await?;
        Ok(height)
    }

    async fn insert_batch(&self, logs: &Vec<(PreDepositFilter, LogMeta)>) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let tx = conn.transaction().await?;
        insert_batch_logs(tx.client(), logs).await?;
        tx.commit().await?;
        Ok(())
    }
}

// Eth1
impl<T: EthSpec> EventService<T> {
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
            .address(self.contract.address().into())
            .query_with_meta()
            .await?;
        Ok(logs)
    }
}

// Eth2
impl<T: EthSpec> EventService<T> {
    async fn get_current_finality_block_number(&self) -> Result<u64> {
        get_current_finality_block_number::<T>(&self.eth2_client)
            .await
            .map_err(|err| anyhow!("{err}"))
    }
}
