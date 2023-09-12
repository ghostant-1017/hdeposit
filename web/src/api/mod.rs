use std::net::SocketAddr;
use std::time::Duration;

use axum::{extract::Query, routing::get, Router};
use axum::{extract::State, Json};
use eth2::types::ChainSpec;
use serde::{Deserialize, Serialize};
mod balance;
mod err;
mod validators;
use err::*;
use slot_clock::SlotClock;
use slot_clock::SystemTimeSlotClock;
use storage::db::PgPool;
use tracing::info;

use crate::api::balance::get_balance;

use self::validators::get_validators;

#[derive(Clone)]
pub struct Server {
    pub pool: PgPool,
    pub clock: SystemTimeSlotClock,
}

impl Server {
    pub fn new(pool: PgPool, clock: SystemTimeSlotClock) -> Self {
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
