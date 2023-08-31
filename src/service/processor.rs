
use std::time::Duration;

use crate::storage::db::PgPool;
use crate::storage::models::{query_pending_deposit_data, StoredDepositData};
use crate::utils::{generate_deposit_calldata, BatchDepositCallData};
use crate::{
    storage::models::{
        insert_deposit_data, query_unflattened_events, query_unused_key_store,
        update_events_to_flattened, update_key_store_fk, StoredKeyStore, StoredPreDepositEvt,
    },
    utils::generate_deposit_data,
};
use anyhow::{ensure, Context, Result};
use bb8_postgres::tokio_postgres::Client;

use ethers::providers::Middleware;


use ethers::types::BlockId;
// use ethers::types::Bytes;
use lighthouse_types::{ChainSpec, DepositData};
use tokio::time::sleep;
use tracing::*;

use super::VaultContract;

const DEPOSIT_AMOUNT: u64 = 32_000_000_000;
pub struct ProcessorService {
    pool: PgPool,
    password: String,
    spec: ChainSpec,
    contract: VaultContract,
}

impl ProcessorService {
    pub fn new(pool: PgPool, password: &str, spec: ChainSpec, contract: VaultContract) -> Self {
        Self {
            pool,
            password: password.to_owned(),
            spec,
            contract,
        }
    }

    pub fn start_update_service(self) -> Result<()> {
        tokio::spawn(async move {
            loop {
                if let Err(err) = self.do_update().await {
                    error!("[Processor] do update error: {:#}", err);
                }
                sleep(Duration::from_secs(12)).await
            }
        });
        Ok(())
    }

    async fn do_update(&self) -> Result<()> {
        info!("[Processor] do update...");
        self.flattern().await?;
        // Preprare calldata to call contract
        self.process().await?;
        // self.contract.deposit(calldata.0, calldata.1, calldata.2, calldata.3, calldata.4).send()
        Ok(())
    }

    /// This method process `PreDepositLog`
    /// 1.flattern the `n` to `DepositData`
    /// 2.update bls_key_store
    async fn flattern(&self) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let tx = conn.transaction().await?;
        let client = tx.client();

        // 1. Select pending events(which's not flattern yet)
        let evts = self
            .select_pending_evts(client)
            .await
            .context("select evts")?;
        let num_evts = evts.len();
        if num_evts == 0 {
            return Ok(());
        }
        info!("[Processor]Found pending events num: {}", evts.len());
        let mut total = 0;
        for evt in evts {
            total += evt.log.n.as_u64();
            debug!("Event expected num: {}", evt.log.n);
            let n = evt.log.n.as_u64();
            // 2. Select `n` unused keystore
            let keys = self
                .select_unused_keystore(client, n)
                .await
                .context("select unused")?;
            // 3. Generate and insert into deposit_data table
            for key in keys {
                let key_pair = key
                    .key_store
                    .decrypt_keypair(self.password.as_bytes())
                    .map_err(|_| anyhow::anyhow!("use password decrypt"))?;
                let deposit_data = generate_deposit_data(
                    &self.spec,
                    &key_pair,
                    &evt.log.withdrawal_credential,
                    DEPOSIT_AMOUNT,
                )
                .context("generate deposit data")?;
                let deposit_data_pk = self
                    .insert_deposit_data(client, &evt, &deposit_data, &key)
                    .await
                    .context("insert deposit data")?;
                // 3. Update keystore foreign key
                self.update_key_store_fk(client, &key, deposit_data_pk)
                    .await
                    .context("update keystore fk")?;
            }
            self.update_events_to_flattened(client, &evt).await?;
        }
        tx.commit().await?;
        info!("[Processor]Successfully flattern events: {num_evts}, deposit data total: {total}.");
        Ok(())
    }

    async fn process(&self) -> Result<()> {
        let conn = self.pool.get().await?;
        let batch_stored: Vec<StoredDepositData> = self.select_pending_deposit_data(&conn).await?;
        let batch_data: Vec<DepositData> = batch_stored
            .into_iter()
            .map(|stored| stored.deposit_data)
            .collect();
        if !batch_data.is_empty() {
            let calldata = generate_deposit_calldata(batch_data);
            self.call_deposit(calldata).await?;
        }
        Ok(())
    }

    async fn call_deposit(&self, calldata: BatchDepositCallData) -> Result<()> {
        info!(
            "[Processor]Prepare to `deposit` with calldata: {}",
            calldata
        );
        let contract_call = self
            .contract
            .deposit(calldata.0, calldata.1, calldata.2, calldata.3, calldata.4);
        let mut tx = contract_call.tx.clone();
        self.contract
            .client()
            .fill_transaction(&mut tx, None)
            .await
            .context("fill transaction")?;
        let from = self.contract.client().address();
        let signature = self.contract.client().sign_transaction(&tx, from).await.context("sign transaction")?;
        let tx_hash = tx.hash(&signature);
        info!("[Processor]Prepare to send transaction: {}", tx_hash);
        Ok(())
    }
}

// DB trait
impl ProcessorService {
    // Asc by block height
    async fn select_pending_evts(&self, client: &Client) -> Result<Vec<StoredPreDepositEvt>> {
        let evts = query_unflattened_events(client).await?;
        Ok(evts)
    }

    async fn select_unused_keystore(&self, client: &Client, n: u64) -> Result<Vec<StoredKeyStore>> {
        let kys = query_unused_key_store(client, n as i64).await?;
        ensure!(
            kys.len() == n as usize,
            "Not enough bls keystore, expect: {}, found: {}.",
            n,
            kys.len()
        );
        Ok(kys)
    }

    async fn insert_deposit_data(
        &self,
        client: &Client,
        evt: &StoredPreDepositEvt,
        deposit_data: &DepositData,
        ks: &StoredKeyStore,
    ) -> Result<i64> {
        let deposit_data_id = insert_deposit_data(client, evt, deposit_data, ks)
            .await
            .context("insert deposit data")?;
        debug!("Insert return deposit data id: {}", deposit_data_id);
        Ok(deposit_data_id)
    }

    async fn select_pending_deposit_data(&self, client: &Client) -> Result<Vec<StoredDepositData>> {
        let batch = query_pending_deposit_data(client).await?;
        Ok(batch)
    }

    async fn update_key_store_fk(
        &self,
        client: &Client,
        ks: &StoredKeyStore,
        fk: i64,
    ) -> Result<()> {
        let result = update_key_store_fk(client, ks, fk).await?;
        ensure!(result == 1, "update bls_keystore fail");
        Ok(())
    }

    async fn update_events_to_flattened(
        &self,
        client: &Client,
        evt: &StoredPreDepositEvt,
    ) -> Result<()> {
        let result = update_events_to_flattened(client, evt.pk).await?;
        ensure!(result == 1, "update pre_deposit_events to flattened error");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_tx_hash() {
        // let contract_addr =
        // Address::from_str(&self.contract).context("parse contract address error")?;
        // let provider = ethers::providers::Provider::try_from(self.eth1_endpoint.as_str())?;
        // let client = Arc::new(SignerMiddleware::new(provider, wallet));
        // let contract = Vault::new(contract_addr, client);
    }
}
