use anyhow::Result;
use hdeposit::cli::Cli;
use clap::Parser;
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.exec().await?;
    Ok(())
}
