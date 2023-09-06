use std::time::Duration;

use eth2::BeaconNodeHttpClient;
use storage::db::PgPool;
use tracing::error;

mod validators;

pub struct Updater {
    beacon: BeaconNodeHttpClient,
    pool: PgPool,
}

impl Updater {
    pub fn new(beacon: BeaconNodeHttpClient, pool: PgPool) -> Self {
        Self { beacon, pool }
    }

    pub async fn run(self) {
        loop {
            if let Err(err) = self.update_validators().await {
                error!("Update validators: {}", err);
            }
            tokio::time::sleep(Duration::from_secs(12)).await
        }
    }
}
