mod cl_reward;
mod el_reward;
mod claim_history;
mod notifier;
mod validator;

use anyhow::anyhow;
use eth2::types::{EthSpec, EventTopic};
use futures::Future;
use storage::db::PgPool;
use tokio::{join, sync::broadcast::error::RecvError};
use tracing::{error, warn};

use crate::component::EthComponent;

use self::notifier::ChainEventTx;

pub async fn run<T: EthSpec>(pool: PgPool, eth: EthComponent) {
    let (event_tx, _) = notifier::init::<T>(eth.beacon.clone());
    let result = join!(
        do_job(
            &event_tx,
            || cl_reward::sync_protocol_rewards::<T>(pool.clone(), eth.clone()),
            EventTopic::FinalizedCheckpoint
        ),
        do_job(
            &event_tx,
            || el_reward::sync_execution_rewards::<T>(pool.clone(), eth.clone()),
            EventTopic::FinalizedCheckpoint
        ),
        do_job(
            &event_tx,
            || claim_history::sync_claim_history::<T>(pool.clone(), eth.clone()),
            EventTopic::FinalizedCheckpoint,
        ),
        do_job(
            &event_tx,
            || validator::sync_validator_info::<T>(pool.clone(), eth.clone()),
            EventTopic::Block,
        )
    );
    error!("result: {:#?}", result);
}

async fn do_job<T: EthSpec, F, Fut>(
    event_tx: &ChainEventTx<T>,
    job: F,
    topic: EventTopic,
) -> anyhow::Result<()>
where
    F: Fn() -> Fut + Send,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    let mut event_rx = event_tx.subscribe();
    if let Err(err) = job().await {
        error!("Do first job error: {err}");
    };
    loop {
        let event = match event_rx.recv().await {
            Ok(event) => event,
            Err(RecvError::Closed) => {
                error!("Event channel closed");
                return Err(anyhow!("Event channel closed"));
            }
            Err(RecvError::Lagged(_)) => {
                warn!("Event channel lagged");
                continue;
            }
        };
        if event.topic_name() != &topic.to_string() {
            continue;
        }
        if let Err(err) = job().await {
            error!("Do job error: {err}");
        }
    }
}
