use std::{net::SocketAddr, sync::Arc, time::Duration};

use crate::{api::Server, logger, updater::Updater};
use clap::Parser;
use contract::deposit::DepositContract;
use eth2::{
    types::{ChainSpec, ConfigAndPreset, MainnetEthSpec, MinimalEthSpec},
    BeaconNodeHttpClient, SensitiveUrl, Timeouts,
};

use storage::db::initial_pg_pool;
use tracing::info;

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[clap(long)]
    eth1_endpoint: String,

    #[clap(long)]
    eth2_endpoint: String,

    #[clap(long)]
    socket: SocketAddr,

    /// Database url
    #[clap(long)]
    dsn: String,

    /// Contract deployed slot
    #[clap(long)]
    start: u64,
}

impl Cli {
    pub async fn exec(self) -> anyhow::Result<()> {
        logger::init(0);
        let beacon = BeaconNodeHttpClient::new(
            SensitiveUrl::parse(&self.eth2_endpoint).expect("invalid eth2_endpoint"),
            Timeouts::set_all(Duration::from_secs(5)),
        );
        let eth1_provider = ethers::providers::Provider::try_from(self.eth1_endpoint.as_str())?;
        let config_and_preset: ConfigAndPreset = beacon
            .get_config_spec()
            .await
            .expect("get config from beacon")
            .data;
        let spec = ChainSpec::from_config::<MainnetEthSpec>(config_and_preset.config())
            .ok_or(anyhow::anyhow!("from config"))?;
        let pool = initial_pg_pool(self.dsn).await?;
        let deposit_contract =
            DepositContract::new(spec.deposit_contract_address, Arc::new(eth1_provider));

        let updater =
            Updater::<MainnetEthSpec>::new(beacon.clone(), pool.clone(), deposit_contract, self.start);

        let server = Server::new(pool, spec);
        tokio::spawn(async move { updater.run().await });
        server.start(self.socket).await?;
        info!("Server closed");
        Ok(())
    }
}
