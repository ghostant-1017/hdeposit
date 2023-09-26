use std::net::SocketAddr;

use axum::routing::post;
use axum::{extract::Path, extract::Query, routing::get, Router};
use axum::{extract::State, Json};
use eth2::types::ChainSpec;
use eth2::BeaconNodeHttpClient;
use serde::{Deserialize, Serialize};
mod balance;
mod claim_history;
mod daily_rewards;
mod err;
mod exit;
mod validators;
mod estimate_rewards;
use err::*;
use slot_clock::SlotClock;
use slot_clock::SystemTimeSlotClock;
use storage::db::PgPool;
use tracing::info;
use crate::api::balance::get_balance;
use crate::api::claim_history::get_claim_history;
use crate::api::daily_rewards::get_daily_rewards_7days;
use crate::api::exit::post_exit;
use crate::api::estimate_rewards::get_estimate_rewards;

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
            .route(
                "/api/v1/wallet/:wc/protocol_rewards",
                get(get_daily_rewards_7days),
            )
            .route("/api/v1/claim_history", get(get_claim_history))
            .route("/api/v1/estimate_rewards", get(get_estimate_rewards))
            .with_state(self);
        info!("Server start at: {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }
}
