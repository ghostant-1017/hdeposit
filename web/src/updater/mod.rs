use std::{marker::PhantomData, time::Duration};

use eth2::{types::EthSpec, BeaconNodeHttpClient};
use storage::db::PgPool;
use tracing::error;

mod validators;
mod withdrawals;
pub struct Updater<T: EthSpec> {
    beacon: BeaconNodeHttpClient,
    pool: PgPool,
    _p: PhantomData<T>,
}

impl<T: EthSpec> Updater<T> {
    pub fn new(beacon: BeaconNodeHttpClient, pool: PgPool) -> Self {
        Self {
            beacon,
            pool,
            _p: Default::default(),
        }
    }

    pub async fn run(self) {
        loop {
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
