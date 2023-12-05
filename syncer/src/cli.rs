use clap::Parser;
use eth2::types::MainnetEthSpec;
use storage::db::initial_pg_pool;
use storage::models::init_sync_states;

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
    
    /// Slot of contract deployed 
    #[clap(long)]
    start: u64,
}

impl Cli {
    pub async fn exec(self) -> anyhow::Result<()> {
        logger::init(0);
        let pool = initial_pg_pool(self.dsn).await?;
        {
            let conn = pool.get().await?;
            init_sync_states(&conn, self.start as i64).await?;
        }
        let eth = EthComponent::new(&self.eth1_endpoint, &self.beacon).await?;
        run::<MainnetEthSpec>(pool, eth).await;
        Ok(())
    }
}
