use anyhow::Result;
use clap::Parser;
use crontab::cli::Cli;
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.exec().await?;
    Ok(())
}
