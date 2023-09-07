use std::{net::SocketAddr, time::Duration};

use clap::Parser;
use eth2::{BeaconNodeHttpClient, SensitiveUrl, Timeouts, Url, types::{ChainSpec, Config, MainnetEthSpec}};
use slot_clock::SlotClock;
use storage::db::initial_pg_pool;
use tracing::info;

use crate::{api::Server, updater::Updater, logger, config::PRATER_CONFIG};

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
        let spec = match self.chain_id {
            0 => ChainSpec::mainnet(),
            _ => {
                let config: Config = serde_yaml::from_str(PRATER_CONFIG)?;
                ChainSpec::from_config::<MainnetEthSpec>(&config).ok_or(anyhow::anyhow!("from config"))?
            }
        };
        // TODO: Init from beacon, drop chain_id
        let clock = slot_clock::SystemTimeSlotClock::new(
            spec.genesis_slot,
            Duration::from_secs(spec.min_genesis_time),
            Duration::from_secs(spec.seconds_per_slot)
        );

        let pool = initial_pg_pool(self.dsn).await?;
        let updater = Updater::new(beacon.clone(), pool.clone());
        let server = Server::new(pool, clock);
        tokio::spawn(async move { updater.run().await });
        server.start(self.socket).await?;
        info!("Server closed");
        Ok(())
    }
}
