use std::net::SocketAddr;

use clap::Parser;
use storage::db::initial_pg_pool;

use crate::component::EthComponent;
use crate::logger;

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[clap(long)]
    eth1_endpoint: String,

    #[clap(long)]
    beacon: String,

    #[clap(long)]
    socket: SocketAddr,

    /// Database url
    #[clap(long)]
    dsn: String,

    #[clap(long)]
    start: u64,

    /// Keystore Password
    #[clap(long)]
    password: String,
}

impl Cli {
    pub async fn exec(self) -> anyhow::Result<()> {
        logger::init(0);
        let _pool = initial_pg_pool(self.dsn).await?;
        let _eth = EthComponent::new(&self.eth1_endpoint, &self.beacon).await?;

        Ok(())
    }
}
