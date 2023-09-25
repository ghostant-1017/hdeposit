mod cl_reward;
mod el_fee;
use std::time::Duration;

use eth2::types::EthSpec;
use storage::db::PgPool;
use tracing::error;

use crate::component::EthComponent;

pub enum TaskState {
    ELFee,
}

pub async fn run<T: EthSpec>(pool: PgPool, eth: EthComponent) {
    loop {
        if let Err(err) = cl_reward::sync_protocol_rewards::<T>(pool.clone(), eth.clone()).await {
            error!("sync protocol rewards: {}", err);
        }
        tokio::time::sleep(Duration::from_secs(12)).await;
    }
}
