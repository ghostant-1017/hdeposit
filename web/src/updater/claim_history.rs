use std::sync::Arc;

use super::*;
use anyhow::anyhow;
use anyhow::Result;
use contract::elfee::ELFee;
use contract::elfee::SplitFeeFilter;
use eth2::types::BlockId;
use ethers::prelude::LogMeta;
use ethers::types::Address;
use storage::models::insert_claim;
use storage::models::query_contract_deployed_block_number;
use storage::models::upsert_sync_state;
use storage::models::{query_all_el_fee_contract, select_sync_state, SyncState};
use tracing::info;
use tracing::warn;

impl<T: EthSpec> Updater<T> {
    pub async fn update_claim_history(&self) -> Result<()> {
        let eth1_client = Arc::new(self.eth1_client.clone());
        let mut db = self.pool.get().await?;
        let el_fee_addresses = query_all_el_fee_contract(&db).await?;
        let to = get_current_finality_block_number::<T>(&self.beacon).await?;
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
                insert_claim(tx.client(), el_fee_address, log, meta).await?;
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
}

pub async fn query_logs_batch(
    contract: ELFee<Provider<Http>>,
    from: u64,
    to: u64,
    el_fee_address: Address,
) -> Result<Vec<(SplitFeeFilter, LogMeta)>> {
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

// TODO: reuse
pub async fn get_current_finality_block_number<T: EthSpec>(
    beacon: &BeaconNodeHttpClient,
) -> Result<u64> {
    let block = beacon
        .get_beacon_blocks::<T>(BlockId::Finalized)
        .await
        .map_err(|err| anyhow!("{err}"))?
        .ok_or(anyhow!("block number not found"))?;
    let payload = block
        .data
        .message()
        .execution_payload()
        .map_err(|err| anyhow!("{err:?}"))?;
    let block_number = payload.execution_payload_ref().block_number();
    Ok(block_number)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use ethers::types::Address;
    #[test]
    fn test_address() {
        let address_str = "0x00005ea00ac477b1030ce78506496e8c2de24bf5";
        let address = Address::from_str(address_str).unwrap();
        let address = serde_json::to_string(&address).unwrap();
        let trimed = address.trim_matches('"');
        assert_eq!(address_str, trimed)
    }
}
