use std::time::Duration;

use crate::eth2::get_current_finality_block_number;
use crate::storage::db::PgPool;
use crate::storage::models::{
    insert_eth_transaction, query_pending_deposit_data, update_batch_deposit_data,
    StoredDepositData, StoredEthTransaction, select_pending_eth_transactions,
};
use crate::utils::{generate_deposit_calldata, BatchDepositCallData};
use crate::{
    storage::models::{
        insert_deposit_data, query_unflattened_events, query_unused_keystore,
        update_events_to_flattened, update_keystore_fk, StoredKeyStore, StoredPreDepositEvt,
    },
    utils::generate_deposit_data,
};
use anyhow::{ensure, Context, Result, anyhow};
use bb8_postgres::tokio_postgres::Client;

use ethers::providers::{Middleware, PendingTransaction, Provider};

use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::{Bytes as EBytes, Signature};
use lighthouse_bls::Hash256;
// use ethers::types::Bytes;
use lighthouse_types::{ChainSpec, DepositData};
use tokio::time::sleep;
use tracing::*;
use url::Url;

use super::VaultContract;

const DEPOSIT_AMOUNT: u64 = 32_000_000_000;
pub struct ProcessorService {
    eth2_endpoint: Url,
    pool: PgPool,
    password: String,
    spec: ChainSpec,
    contract: VaultContract,
    provider: Provider<ethers::providers::Http>,
}

impl ProcessorService {
    pub fn new(eth2_endpoint: Url, pool: PgPool, password: &str, spec: ChainSpec, contract: VaultContract, provider: Provider<ethers::providers::Http>) -> Self {
        Self {
            eth2_endpoint,
            pool,
            password: password.to_owned(),
            spec,
            contract,
            provider
        }
    }

    pub fn start_update_service(self) -> Result<()> {
        tokio::spawn(async move {
            loop {
                if let Err(err) = self.confirm_pending_tx().await {
                    error!("[Processor]Confirm pending tx error: {}" ,err);
                    sleep(Duration::from_secs(12)).await;
                    continue;
                }
                if let Err(err) = self.do_update().await {
                    error!("[Processor] do update error: {:#}", err);
                }
                sleep(Duration::from_secs(12)).await;
            }
        });
        Ok(())
    }

    async fn do_update(&self) -> Result<()> {
        info!("[Processor] do update...");
        // 1.Flattern `PreDepositFilter` to `DepositData`
        self.flattern().await?;
        // 2.Preprare calldata and generate raw_tx
        self.process().await?;

        Ok(())
    }

    /// This method process `PreDepositLog`
    /// 1.flattern the `n` to `DepositData`
    /// 2.update bls_keystore
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
                    .keystore
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
                self.update_keystore_fk(client, &key, deposit_data_pk)
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
        let mut conn = self.pool.get().await?;
        let db_tx = conn.transaction().await?;
        // 1.Check if the deposit_data is enough
        let batch_stored = match self.prepare_batch_deposit_data(db_tx.client()).await? {
            Some(batch) => batch,
            None => return Ok(()),
        };

        let batch_data: Vec<DepositData> = batch_stored
            .iter()
            .map(|stored| stored.deposit_data.clone())
            .collect();
        // 2. Generate deposit_calldata
        let calldata = generate_deposit_calldata(&batch_data);
        // 3. Insert the transaction and signature ralated to eth_tx
        let (tx, signature) = self.prepare_tx_and_signature(calldata).await?;
        let eth_tx_pk = self
            .insert_eth_transaction(db_tx.client(), tx, signature)
            .await?;
        self.update_deposit_data_with_eth_tx_pk(db_tx.client(), batch_stored, eth_tx_pk)
            .await?;

        db_tx.commit().await?;
        Ok(())
    }

    async fn confirm_pending_tx(&self) -> Result<()> {
        let conn = self.pool.get().await?;
        let eth_tx: StoredEthTransaction = match self.select_eth_transactions(&conn).await? {
            Some(eth_tx) => eth_tx,
            None => return Ok(())
        };
        info!("Found pending transaction: {}", eth_tx.tx_hash.to_string());
        info!("Prepare send raw data: {}", eth_tx.raw_tx());
        let pending_tx = self.send_raw_transaction(eth_tx.raw_tx()).await?;
        ensure!(pending_tx == eth_tx.tx_hash, "transaction hash not match");
        self.wait_for_finality(eth_tx.tx_hash).await?;
        Ok(())
    }

    async fn wait_for_finality(&self, tx_hash: Hash256) -> Result<()> {
        let eth_client = self.contract.client();
        let provider = eth_client.provider();
        loop {
            let pending_tx = PendingTransaction::new(tx_hash, provider);
            let receipt = pending_tx.await?.ok_or(anyhow!("Transaction not found"))?;
            let finality = get_current_finality_block_number(&self.eth2_endpoint).await?;
            if receipt.block_number.ok_or(anyhow!("block number not found"))? <= finality.into() {
                return Ok(())
            }
        }
    } 

    async fn prepare_batch_deposit_data(
        &self,
        client: &Client,
    ) -> Result<Option<Vec<StoredDepositData>>> {
        let batch_stored = self.select_pending_deposit_data(client).await?;
        // TODO: check the number
        if batch_stored.is_empty() {
            return Ok(None);
        }
        Ok(Some(batch_stored))
    }

    async fn prepare_tx_and_signature(
        &self,
        calldata: BatchDepositCallData,
    ) -> Result<(TypedTransaction, Signature)> {
        info!(
            "[Processor]Prepare to `deposit` with calldata: {}",
            calldata
        );
        let contract_call = self
            .contract
            .deposit(calldata.0, calldata.1, calldata.2, calldata.3, calldata.4);
        let mut tx = contract_call.tx.clone();
        let eth_client = self.contract.client();
        eth_client
            .fill_transaction(&mut tx, None)
            .await
            .context("fill transaction")?;

        let from = eth_client.address();
        let signature = eth_client
            .sign_transaction(&tx, from)
            .await
            .context("sign transaction")?;
        info!("Signed raw_tx: {}", tx.rlp_signed(&signature).to_string());
        info!("TransactionHash: {}", tx.hash(&signature));
        info!("Transaction: {}", serde_json::to_string(&tx)?);
        // let tx_hash = tx.hash(&signature);
        // info!("[Processor]Prepare to send transaction: {}", tx_hash.to_string());
        // let raw_tx = tx.rlp_signed(&signature);
        Ok((tx, signature))
        // Send transaction after update
        // let pending_tx = self.provider.send_raw_transaction(raw_tx).await.context("send raw transaction")?;
        // ensure!(pending_tx.tx_hash() == tx_hash, "Transaction hash didn't match!");
    }

    async fn send_raw_transaction(&self, raw_tx: EBytes) -> Result<Hash256> {
        // let eth_client = self.contract.client();
        // let pending_tx = eth_client.send_raw_transaction(raw_tx).await?;
        let pending_tx = self.provider.send_raw_transaction(raw_tx).await?;
        Ok(pending_tx.to_owned())
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
        let kys = query_unused_keystore(client, n as i64).await?;
        ensure!(
            kys.len() == n as usize,
            "Not enough bls keystore, expect: {}, found: {}.",
            n,
            kys.len()
        );
        Ok(kys)
    }

    async fn select_eth_transactions(&self, client: &Client) -> Result<Option<StoredEthTransaction>> {
        let mut txs = select_pending_eth_transactions(client).await?;
        ensure!(txs.len() <= 1, "Critical bug, pending transactions in db should be less than 1, found: {}", txs.len());
        if txs.is_empty() {
            return Ok(None)
        }
        return Ok(Some(txs.pop().unwrap()))
    }

    async fn insert_eth_transaction(
        &self,
        client: &Client,
        tx: TypedTransaction,
        signature: Signature,
    ) -> Result<i64> {
        let eth_transaction_pk = insert_eth_transaction(client, tx, signature).await?;
        Ok(eth_transaction_pk)
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

    async fn update_keystore_fk(
        &self,
        client: &Client,
        ks: &StoredKeyStore,
        fk: i64,
    ) -> Result<()> {
        let result = update_keystore_fk(client, ks, fk).await?;
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

    async fn update_deposit_data_with_eth_tx_pk(
        &self,
        client: &Client,
        batch: Vec<StoredDepositData>,
        eth_tx_pk: i64,
    ) -> Result<()> {
        let result = update_batch_deposit_data(client, &batch, eth_tx_pk).await?;
        ensure!(
            result == batch.len() as u64,
            "Critical bug when set eth_tx_pk, expect: {}, found: {}",
            batch.len(),
            result
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use ethers::{types::Bytes, utils::hex::FromHex, providers::PendingTransaction};
    use lighthouse_types::Hash256;


    #[test]
    fn test_tx_hash() {
        // let contract_addr =
        // Address::from_str(&self.contract).context("parse contract address error")?;
        // let provider = ethers::providers::Provider::try_from(self.eth1_endpoint.as_str())?;
        // let client = Arc::new(SignerMiddleware::new(provider, wallet));
        // let contract = Vault::new(contract_addr, client);
    }
    #[tokio::test]
    async fn test_pending_tx() {
        env::set_var("http_proxy", "http://127.0.0.1:59527");
        env::set_var("https_proxy", "http://127.0.0.1:59527");
        let tx_hex = "0x801b4041772a56b891537d57c01298a582a90b003fe4eef5dd09b624bb11174a";
        let eth1_endpoint = "https://eth.getblock.io/310a66fb-9df2-4436-a22f-b7d7d28092e9/goerli/";
        let provider = ethers::providers::Provider::try_from(eth1_endpoint).unwrap();
        let tx_hash = Bytes::from_hex(tx_hex).unwrap();
        let tx_hash = Hash256::from_slice(&tx_hash);
        let pending_tx = PendingTransaction::new(tx_hash, &provider).confirmations(12);
        let receipt = pending_tx.await.unwrap();
        println!("{:?}", receipt);
    }
}
