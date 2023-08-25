use std::sync::Arc;

use crate::vault::{PreDepositFilter, Vault};
use anyhow::Result;
use ethers::prelude::SignerMiddleware;
use ethers::{
    providers::{Http, Provider},
    signers::{LocalWallet, Wallet},
    types::Address,
};
use ethers::prelude::LogMeta;
use k256::ecdsa::SigningKey;
pub struct EventService {
    contract: Vault<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
}

impl EventService {
    pub fn new(endpoint: &str, contract_addr: Address, wallet: LocalWallet) -> Result<Self> {
        let provider = ethers::providers::Provider::try_from(endpoint)?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = Vault::new(contract_addr, client);
        Ok(Self { contract })
    }

    pub fn start_update_service(self) -> Result<()> {
        tokio::spawn(async move {
            loop {
                self.do_update();
            }
        });
        Ok(())
    }

    async fn do_update(&self) -> Result<()> {
        Ok(())
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
