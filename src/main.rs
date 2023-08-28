use anyhow::{Context, Result};
use clap::Parser;
use ethers::types::Address;
use url::Url;
use std::str::FromStr;
mod db;
mod logger;
mod processor;
mod syncer;
mod utils;
mod vault;
mod wallet;
mod model;
mod eth2;
use crate::{syncer::EventService, wallet::inital_wallet_from_env, db::initial_pg_pool};
use tracing::*;

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
}

#[tokio::main]
async fn main() -> Result<()> {
    logger::init(0);
    let cli = Cli::parse();
    info!(?cli);
    info!("Loading contract owner secret key from env...");
    let wallet = inital_wallet_from_env().context("init local wallet fail")?;
    info!("Initializing db connection pool...");
    let pool = initial_pg_pool(cli.dsn).await?;
    let contract_addr = Address::from_str(&cli.contract).context("parse contract address error")?;

    let evt_service = EventService::new(cli.eth1_endpoint,cli.eth2_endpoint, contract_addr, wallet, pool)?;
    let _ = evt_service.start_update_service(cli.start).await?;
    Ok(())
}
