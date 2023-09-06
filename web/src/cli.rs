use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use eth2::{BeaconNodeHttpClient, SensitiveUrl, Timeouts, Url};
use storage::db::initial_pg_pool;

use crate::{api::Server, updater::Updater, logger};

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[clap(long)]
    eth2_endpoint: String,

    #[clap(long)]
    socket: SocketAddr,

    #[clap(long)]
    chain_id: i8,

    /// Database url
    #[clap(long)]
    dsn: String,
}

impl Cli {
    pub async fn exec(self) -> anyhow::Result<()> {
        logger::init(0);
        let beacon = BeaconNodeHttpClient::new(
            SensitiveUrl::parse(&self.eth2_endpoint).expect("invalid eth2_endpoint"),
            Timeouts::set_all(Duration::from_secs(5)),
        );
        let pool = initial_pg_pool(self.dsn).await?;
        let updater = Updater::new(beacon.clone(), pool.clone());
        let server = Server::new(pool);
        tokio::spawn(async move { updater.run().await });
        server.start(self.socket).await?;
        Ok(())
    }
}
