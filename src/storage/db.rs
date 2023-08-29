use anyhow::Context;
use anyhow::Result;
use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::{bb8, tokio_postgres::NoTls, PostgresConnectionManager};
pub type PgPool = bb8::Pool<PostgresConnectionManager<NoTls>>;
pub type PgConnection<'a> = PooledConnection<'a, PostgresConnectionManager<NoTls>>;

pub async fn initial_pg_pool(dsn: String) -> Result<PgPool> {
    let mgr = PostgresConnectionManager::new(
        dsn.parse().context("convert pg dsn to connect config")?,
        NoTls,
    );
    bb8::Pool::builder()
        .max_size(100)
        .build(mgr)
        .await
        .context("construct db client pool")
}
