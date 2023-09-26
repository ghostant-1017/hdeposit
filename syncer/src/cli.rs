use clap::Parser;
use eth2::types::MainnetEthSpec;
use storage::db::initial_pg_pool;

use crate::component::EthComponent;
use crate::logger;
use crate::tasks::run;

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[clap(long)]
    eth1_endpoint: String,

    #[clap(long)]
    beacon: String,

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
        let pool = initial_pg_pool(self.dsn).await?;
        let eth = EthComponent::new(&self.eth1_endpoint, &self.beacon).await?;
        run::<MainnetEthSpec>(pool, eth).await;
        Ok(())
    }
}
