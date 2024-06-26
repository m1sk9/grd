use clap::{Parser, Subcommand};

mod download;

#[derive(Parser)]
#[command(
    name = "grd",
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION")
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install the deliverables from the release page of the specified repository.
    Download(download::DownloadCmd),
}

impl Cli {
    pub async fn run(self) -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .compact()
            .with_target(false)
            .without_time()
            .init();

        use Command::*;
        match self.command {
            Download(cmd) => cmd.run().await,
        }
    }
}
