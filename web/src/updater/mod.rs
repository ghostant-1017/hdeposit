use std::{marker::PhantomData, time::Duration};

use contract::deposit::DepositContract as DepositContractABI;
use eth2::{types::EthSpec, BeaconNodeHttpClient};
use ethers::providers::Http;
use ethers::providers::Provider;
use storage::db::PgPool;
use tracing::error;
type DepositContract = DepositContractABI<Provider<Http>>;

mod deposit_events;
mod validators;
mod withdrawals;
pub struct Updater<T: EthSpec> {
    beacon: BeaconNodeHttpClient,
    pool: PgPool,
    deposit_contract: DepositContract,
    start: u64,
    _p: PhantomData<T>,
}

impl<T: EthSpec> Updater<T> {
    pub fn new(
        beacon: BeaconNodeHttpClient,
        pool: PgPool,
        deposit_contract: DepositContract,
        start: u64
    ) -> Self {
        Self {
            beacon,
            pool,
            deposit_contract,
            start,
            _p: Default::default(),
        }
    }

    pub async fn run(self) {
        loop {
            if let Err(err) = self.update_deposit_events().await {
                error!("Update deposit_events: {}", err);
            }
            if let Err(err) = self.update_validators().await {
                error!("Update validators: {}", err);
            }
            if let Err(err) = self.update_withdrawals().await {
                error!("Update withdrawls: {}", err);
            }
            tokio::time::sleep(Duration::from_secs(12)).await
        }
    }
}
