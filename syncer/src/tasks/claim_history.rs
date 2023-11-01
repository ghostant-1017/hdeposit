use std::sync::Arc;

use eth2::types::EthSpec;
use ethers::{types::Address, providers::Middleware};
use ethers::prelude::LogMeta;
use storage::{db::PgPool, models::{query_all_el_fee_contract, SyncState, select_sync_state, query_contract_deployed_block_number, insert_claim, upsert_sync_state}};
use tracing::{info, warn};
use contract::elfee::{ELFee, SplitFeeFilter};

use crate::{component::{EthComponent, self}, beacon::get_current_finality_block_number, geth::Eth1Client};


pub async fn sync_claim_history<T: EthSpec>(
    pool: PgPool,
    eth: EthComponent,
) -> anyhow::Result<()> {
    let eth1_client = Arc::new(eth.eth1.clone());
    let mut db = pool.get().await?;
    let el_fee_addresses = query_all_el_fee_contract(&db).await?;
    let to = get_current_finality_block_number::<T>(&eth.beacon).await?;
    info!(
        "Prepare to update el fee address: {}",
        el_fee_addresses.len()
    );
    for el_fee_address in el_fee_addresses {
        if el_fee_address.is_zero() {
            continue;
        }
        let tx = db.transaction().await?;
        let from =
            match select_sync_state(tx.client(), &SyncState::ContractLogs(el_fee_address))
                .await?
            {
                Some(from) => from,
                None => {
                    let block_number =
                        query_contract_deployed_block_number(tx.client(), el_fee_address)
                            .await?;
                    if block_number.is_none() {
                        warn!(
                            "Cannot find el fee contract deploy number: {}",
                            el_fee_address
                        );
                        continue;
                    }
                    block_number.unwrap()
                }
            };

        if from == to {
            continue;
        }
        let contract = ELFee::new(el_fee_address, eth1_client.clone());
        let logs = query_logs_batch(contract, from, to, el_fee_address).await?;
        for (log, meta) in logs {
            let block_timestamp = eth.eth1
            .get_block(meta.block_hash)
            .await?
            .ok_or(anyhow::anyhow!("block not found"))?
            .timestamp;
            insert_claim(tx.client(), el_fee_address, log, meta, block_timestamp.as_u64() as i64).await?;
        }
        upsert_sync_state(
            tx.client(),
            &SyncState::ContractLogs(el_fee_address),
            &(to as i64),
        )
        .await?;
        tx.commit().await?;
    }
    Ok(())
}

pub async fn query_logs_batch(
    contract: ELFee<Arc<Eth1Client>>,
    from: u64,
    to: u64,
    el_fee_address: Address,
) -> anyhow::Result<Vec<(SplitFeeFilter, LogMeta)>> {
    let mut result = vec![];
    for i in (from..=to).step_by(10000) {
        let current_from = i;
        let current_to = (i + 10000).min(to);
        info!("Query split fee logs from {current_from} to {current_to}");
        let logs = contract
            .split_fee_filter()
            .address(el_fee_address.into())
            .from_block(current_from)
            .to_block(current_to)
            .query_with_meta()
            .await?;
        result.extend(logs);
    }
    Ok(result)
}
