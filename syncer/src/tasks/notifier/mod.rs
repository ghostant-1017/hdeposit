use std::sync::Arc;

use super::*;
use crate::beacon::BeaconClient;
use anyhow::anyhow;
use eth2::types::{EventKind, EventTopic};
use futures::StreamExt;
use tokio::sync::broadcast::channel;
use tokio::sync::broadcast::{Receiver, Sender};
use tracing::{info, warn};

pub type ChainEventTx<T> = Sender<EventKind<T>>;
pub type ChainEventRx<T> = Receiver<EventKind<T>>;

pub fn init<T: EthSpec>(beacon: Arc<BeaconClient>) -> (ChainEventTx<T>, ChainEventRx<T>) {
    let (tx, rx) = channel(1024);
    let mut event_topics = vec![];
    event_topics.push(EventTopic::FinalizedCheckpoint);
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        loop {
            let mut stream = match beacon.get_events::<T>(&event_topics).await {
                Ok(stream) => stream,
                Err(err) => {
                    error!("get events stream error: {err}");
                    continue;
                }
            };
            while let Some(event) = stream.next().await {
                let event = match event {
                    Ok(event) => event,
                    Err(_) => {
                        continue;
                    }
                };
                info!("Chain event coming: {:?}", event);
                let _ = tx.send(event);
            }
            warn!("Beacon events stream broken")
        }
    });
    (tx_clone, rx)
}
