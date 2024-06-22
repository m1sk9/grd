use clap::{Parser, Subcommand};

mod install;

#[derive(Parser)]
#[command(
    name = "grd",
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION")
)]
pub struct GrdCli {
    #[command(subcommand)]
    pub command: GrdCliCommand,
}

#[derive(Debug, Subcommand)]
pub enum GrdCliCommand {
    Install(install::InstallCmd),
}

impl GrdCli {
    pub async fn run(self) -> anyhow::Result<()> {
        use GrdCliCommand::*;
        match self.command {
            Install(cmd) => cmd.run().await,
        }
    }
}
