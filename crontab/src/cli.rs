use crate::{
    eth2::{new_beacon_client, new_validator_client},
    logger,
    service::{EventService, ProcessorService},
    storage::db::initial_pg_pool,
    vault::Vault,
    wallet::inital_wallet_from_env,
};
use anyhow::{anyhow, ensure, Context, Result};
use clap::Parser;
use ethers::prelude::SignerMiddleware;
use ethers::types::Address;
use lighthouse_types::{ChainSpec, EthSpec, MainnetEthSpec, ConfigAndPreset};
use std::{str::FromStr, sync::Arc};
use tracing::*;
use url::Url;
#[derive(Parser, Clone, Debug)]
pub struct Cli {
    /// The execution layer api endpoitn
    #[clap(long)]
    eth1_endpoint: Url,

    #[clap(long)]
    beacon_endpoint: Url,

    #[clap(long)]
    validator_endpoint: Url,

    #[clap(long)]
    validator_secret: String,

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

    #[clap(long, default_value = "1")]
    batch: u64,
}

impl Cli {
    pub async fn exec(self) -> Result<()> {
        logger::init(0);
        info!("Loading contract owner secret key from env[CONTRACT_OWNER_KEY]...");
        let wallet = inital_wallet_from_env().context("init local wallet fail")?;
        
        info!("Initializing db connection pool...");
        let pool = initial_pg_pool(self.dsn).await?;
        
        info!("Initializing beacon http client...");
        let beacon_client = new_beacon_client(self.beacon_endpoint.as_str())?;
        
        info!("Initializing validator http client...");
        let validator_client = new_validator_client(self.validator_endpoint.as_str(), self.validator_secret)?;

        info!("Loading chain spec from beacon node...");
        let config_and_preset: ConfigAndPreset = beacon_client
            .get_config_spec()
            .await
            .expect("get config from beacon")
            .data;
        let spec = ChainSpec::from_config::<MainnetEthSpec>(config_and_preset.config())
            .ok_or(anyhow::anyhow!("from config"))?;
        info!("Loaded config from beacon, config_name: {}", spec.config_name.as_ref().unwrap());

        let contract_addr =
            Address::from_str(&self.contract).context("parse contract address error")?;
        let provider = ethers::providers::Provider::try_from(self.eth1_endpoint.as_str())?;
        let client = Arc::new(SignerMiddleware::new_with_provider_chain(provider, wallet).await?);
        let contract = Vault::new(contract_addr, client);
        ensure!(self.batch <= 50, "Batch should less than 50");
        let evt_service = EventService::<MainnetEthSpec>::new(
            beacon_client.clone(),
            contract.clone(),
            pool.clone(),
        )?;
        let proc_service = ProcessorService::new(
            beacon_client.clone(),
            pool,
            &self.password,
            spec,
            contract,
            self.batch,
        );
        run(self.start, evt_service, proc_service).await?;
        Ok(())
    }
}

async fn run<T: EthSpec>(
    start: u64,
    evt_service: EventService<T>,
    proc_service: ProcessorService<T>,
) -> Result<()> {
    proc_service.start_update_service()?;
    evt_service.start_update_service(start).await?;
    Ok(())
}
