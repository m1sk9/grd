use clap::Parser;
use cmd::GrdCli;

mod cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let result = GrdCli::parse().run().await;
    if let Err(why) = result {
        eprintln!("Error: {}", why);
        std::process::exit(1);
    }
    Ok(())
}
