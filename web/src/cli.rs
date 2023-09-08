use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use eth2::{
    types::{ChainSpec, ConfigAndPreset, MainnetEthSpec},
    BeaconNodeHttpClient, SensitiveUrl, Timeouts,
};
use slot_clock::SlotClock;
use storage::db::initial_pg_pool;
use tracing::info;

use crate::{api::Server, logger, updater::Updater};

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[clap(long)]
    eth2_endpoint: String,

    #[clap(long)]
    socket: SocketAddr,

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
        let config_and_preset: ConfigAndPreset = beacon
            .get_config_spec()
            .await
            .expect("get config from beacon")
            .data;
        let spec = ChainSpec::from_config::<MainnetEthSpec>(config_and_preset.config())
            .ok_or(anyhow::anyhow!("from config"))?;
        let pool = initial_pg_pool(self.dsn).await?;
        let updater = Updater::<MainnetEthSpec>::new(beacon.clone(), pool.clone());

        let server = Server::new(pool, spec);
        tokio::spawn(async move { updater.run().await });
        server.start(self.socket).await?;
        info!("Server closed");
        Ok(())
    }
}
