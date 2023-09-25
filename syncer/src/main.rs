use clap::Parser;
use syncer::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.exec().await?;
    Ok(())
}
