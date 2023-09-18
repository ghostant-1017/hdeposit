use std::net::SocketAddr;
use std::time::Duration;

use axum::routing::post;
use axum::{extract::Query, routing::get, Router};
use axum::{extract::State, Json};
use eth2::types::ChainSpec;
use eth2::BeaconNodeHttpClient;
use serde::{Deserialize, Serialize};
mod balance;
mod err;
mod exit;
mod validators;
use err::*;
use slot_clock::SlotClock;
use slot_clock::SystemTimeSlotClock;
use storage::db::PgPool;
use tracing::info;

use crate::api::balance::get_balance;
use crate::api::exit::post_exit;

use self::validators::get_validators;

#[derive(Clone)]
pub struct Server {
    pub pool: PgPool,
    pub clock: SystemTimeSlotClock,
    pub spec: ChainSpec,
    pub password: String,
    pub beacon: BeaconNodeHttpClient,
}

impl Server {
    pub fn new(
        pool: PgPool,
        clock: SystemTimeSlotClock,
        spec: ChainSpec,
        password: String,
        beacon: BeaconNodeHttpClient,
    ) -> Self {
        Self {
            pool,
            clock,
            spec,
            password,
            beacon,
        }
    }

    pub async fn start(self, addr: SocketAddr) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/api/v1/exit", post(post_exit))
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
