use std::net::SocketAddr;

use axum::{extract::Query, routing::get, Router};
mod err;
mod validators;
use err::*;
use storage::db::PgPool;
use tracing::info;

use self::validators::get_validators;

#[derive(Clone)]
pub struct Server {
    pub pool: PgPool,
}

impl Server {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn start(self, addr: SocketAddr) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/api/v1/validators", get(get_validators))
            .with_state(self);
        info!("Server start at: {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}
