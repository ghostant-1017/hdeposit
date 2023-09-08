use std::net::SocketAddr;
use std::time::Duration;

use axum::{extract::State, Json};
use axum::{extract::Query, routing::get, Router};
use eth2::types::ChainSpec;
use serde::{Serialize, Deserialize};
mod err;
mod validators;
mod balance;
use err::*;
use slot_clock::SystemTimeSlotClock;
use storage::db::PgPool;
use tracing::info;
use slot_clock::SlotClock;


use crate::api::balance::get_balance;

use self::validators::get_validators;

#[derive(Clone)]
pub struct Server {
    pub pool: PgPool,
    pub clock: SystemTimeSlotClock
}

impl Server {
    pub fn new(pool: PgPool, spec: ChainSpec) -> Self {
        let clock = slot_clock::SystemTimeSlotClock::new(
            spec.genesis_slot,
            Duration::from_secs(spec.min_genesis_time),
            Duration::from_secs(spec.seconds_per_slot)
        );
        Self { pool, clock }
    }

    pub async fn start(self, addr: SocketAddr) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/api/v1/validators", get(get_validators))
            .route("/api/v1/balance", get(get_balance))
            .with_state(self);
        info!("Server start at: {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }
}
