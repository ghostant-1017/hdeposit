use anyhow::{Context, Result};
use clap::Parser;
use ethers::types::Address;
use std::str::FromStr;
mod db;
mod logger;
mod processor;
mod syncer;
mod utils;
mod vault;
mod wallet;
use crate::{syncer::EventService, wallet::inital_wallet_from_env};
use tracing::*;

#[derive(Parser, Clone, Debug)]
pub struct Cli {
    /// The execution layer api endpoitn
    #[clap(long)]
    endpoint: String,

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
    let cli = Cli::parse();
    info!(?cli);
    info!("Loading contract owner secret key from env...");
    let wallet = inital_wallet_from_env().context("init local wallet fail")?;
    let contract_addr = Address::from_str(&cli.contract).context("contract address error")?;

    let evt_service = EventService::new(&cli.endpoint, contract_addr, wallet)?;
    let _ = evt_service.start_update_service();
    Ok(())
}
