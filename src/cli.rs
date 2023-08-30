use crate::{
    config::PRATER_CONFIG,
    logger,
    service::{EventService, ProcessorService},
    storage::db::initial_pg_pool,
    vault::Vault,
    wallet::inital_wallet_from_env,
};
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use ethers::prelude::SignerMiddleware;
use ethers::types::Address;
use lighthouse_types::{ChainSpec, Config, MainnetEthSpec};
use std::{str::FromStr, sync::Arc};
use tracing::*;
use url::Url;
#[derive(Parser, Clone, Debug)]
pub struct Cli {
    /// The execution layer api endpoitn
    #[clap(long)]
    eth1_endpoint: Url,

    #[clap(long)]
    eth2_endpoint: Url,

    /// Contract address
    #[clap(long)]
    contract: String,

    /// Block height of contract's deloyed
    #[clap(long)]
    start: u64,

    /// Database url
    #[clap(long)]
    dsn: String,

    /// Password of keystore
    #[clap(long)]
    password: String,

    /// ChainID
    /// mainnet: 0
    /// geriol: 1
    #[clap(long)]
    chain_id: i8,
}

impl Cli {
    pub async fn exec(self) -> Result<()> {
        logger::init(0);
        info!("Loading contract owner secret key from env[CONTRACT_OWNER_KEY]...");
        let wallet = inital_wallet_from_env().context("init local wallet fail")?;
        info!("Initializing db connection pool...");
        let pool = initial_pg_pool(self.dsn).await?;
        let contract_addr =
            Address::from_str(&self.contract).context("parse contract address error")?;
        info!("Starting event service...");
        let provider = ethers::providers::Provider::try_from(self.eth1_endpoint.as_str())?;
        let client = Arc::new(SignerMiddleware::new(provider, wallet));
        let contract = Vault::new(contract_addr, client);

        let evt_service = EventService::new(self.eth2_endpoint, contract.clone(), pool.clone())?;
        let spec = match self.chain_id {
            0 => ChainSpec::mainnet(),
            _ => {
                let config: Config = serde_yaml::from_str(PRATER_CONFIG)?;
                ChainSpec::from_config::<MainnetEthSpec>(&config).ok_or(anyhow!("from config"))?
            }
        };
        let proc_service = ProcessorService::new(pool, &self.password, spec, contract);
        run(self.start, evt_service, proc_service).await?;
        Ok(())
    }
}

async fn run(start: u64, evt_service: EventService, proc_service: ProcessorService) -> Result<()> {
    proc_service.start_update_service()?;
    evt_service.start_update_service(start).await?;
    Ok(())
}
