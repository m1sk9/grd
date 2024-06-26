use clap::Parser;
use cmd::Cli;
use tracing::error;

mod assets;
mod cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = Cli::parse().run().await;
    if let Err(why) = result {
        error!("{}", why);
        std::process::exit(1);
    }
    Ok(())
}
