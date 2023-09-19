use super::*;
use anyhow::Result;
use contract::elfee::ELFee;
use storage::models::query_all_el_fee_contract;
use tracing::info;

impl<T: EthSpec> Updater<T> {
    pub async fn update_claim_history(&self) -> Result<()> {
        let db = self.pool.get().await?;
        let el_fee_addresses = query_all_el_fee_contract(&db).await?;
        info!("Prepare to update el fee address: {}", el_fee_addresses.len());
        Ok(())
    }
}