use super::*;
use anyhow::Result;
use contract::deposit::DepositEventFilter;

use ethers::prelude::LogMeta;
use ethers::{providers::Middleware, types::H256};
use storage::models::{
    select_pending_eth_txs_by_gt_pk, select_sync_state, upsert_sync_state,
    upsert_validators_by_logs, SyncState,
};

impl<T: EthSpec> Updater<T> {
    // Query the pending deposit events and update `hellman_validators` table
    pub async fn update_deposit_events(&self) -> Result<()> {
        let mut conn = self.pool.get().await?;
        let db_tx = conn.transaction().await?;
        let tx_pk = select_sync_state(db_tx.client(), &SyncState::DepositTxLastPK)
            .await?
            .unwrap_or_default();
        let txs = select_pending_eth_txs_by_gt_pk(db_tx.client(), tx_pk as i64).await?;
        if txs.is_empty() {
            return Ok(());
        }
        let mut current_pk = 0;
        for tx in txs {
            let logs = match get_logs_by_txhash(self.deposit_contract.clone(), tx.tx_hash).await? {
                Some(logs) => logs.into_iter().map(|log| log.0).collect(),
                None => {
                    tracing::warn!(
                        "Cannot find transaction: {} ralated deposit events.",
                        tx.tx_hash
                    );
                    break;
                }
            };
            current_pk = tx.pk;
            // Insert into validators table by logs;
            upsert_validators_by_logs(db_tx.client(), &logs).await?;
        }
        if current_pk > tx_pk as i64 {
            upsert_sync_state(db_tx.client(), &SyncState::DepositTxLastPK, &current_pk).await?;
        }
        db_tx.commit().await?;
        Ok(())
    }
}

pub async fn get_logs_by_txhash(
    deposit_contract: DepositContract,
    txhash: H256,
) -> Result<Option<Vec<(DepositEventFilter, LogMeta)>>> {
    let client = deposit_contract.client();
    let transaction = client.get_transaction(txhash).await?;
    let block_hash = match transaction.and_then(|tx| tx.block_hash) {
        Some(block_hash) => block_hash,
        None => return Ok(None),
    };
    let address = deposit_contract.address();
    let logs: Vec<(DepositEventFilter, LogMeta)> = deposit_contract
        .deposit_event_filter()
        .at_block_hash(block_hash)
        .address(address.into())
        .query_with_meta()
        .await?;
    Ok(Some(logs))
}
