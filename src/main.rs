use anyhow::{Context, Result};
use clap::Parser;
use ethers::{signers::LocalWallet, types::Address, utils::hex};
use k256::elliptic_curve::generic_array::GenericArray;
use std::{env, str::FromStr};
mod vault;
mod eth1;
use ethers::prelude::Abigen;

use crate::{vault::PreDepositFilter, eth1::EventService};
#[derive(Parser, Clone)]
pub struct Cli {
    #[clap(long)]
    endpoint: String,

    #[clap(long)]
    contract: String,

    #[clap(long)]
    start: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let wallet = inital_wallet_from_env().context("init local wallet fail")?;
    let contract_addr = Address::from_str(&cli.contract).context("contract address error")?;
    let eth1 = EventService::new(&cli.endpoint, contract_addr, wallet)?;
    println!("Query...");
    let logs = eth1.query_pre_deposit_logs(cli.start, 9600000).await?;
    println!("{:?}", logs);

    Ok(())
}

fn inital_wallet_from_env() -> Result<LocalWallet> {
    let secret_key = env::var("CONTRACT_OWNER_KEY")?;
    let key_hex = hex::decode(secret_key)?;
    let key = k256::SecretKey::from_bytes(&GenericArray::clone_from_slice(&key_hex))?;
    let wallet = key.into();
    Ok(wallet)
}

fn rust_file_generation() -> Result<()> {
    let abi_source = "./abi/Vault.abi";
    let out_file = "./test.out";

    Abigen::new("Vault", abi_source).unwrap().generate().unwrap().write_to_file(out_file).unwrap();
    Ok(())
}
