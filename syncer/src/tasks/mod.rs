mod cl_reward;
mod el_reward;
mod notifier;
use std::time::Duration;

use eth2::types::EthSpec;
use storage::db::PgPool;
use tracing::error;

use crate::component::EthComponent;

pub async fn run<T: EthSpec>(pool: PgPool, eth: EthComponent) {
    let (event_tx, event_rx) = notifier::init::<T>(eth.beacon.clone());
    loop {
        if let Err(err) = cl_reward::sync_protocol_rewards::<T>(pool.clone(), eth.clone()).await {
            error!("sync protocol rewards: {}", err);
        }
        tokio::time::sleep(Duration::from_secs(12)).await;
    }
}
