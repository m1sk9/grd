use std::env;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "grd",
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION")
)]
struct GrdCli {
    #[command(subcommand)]
    command: GrdCliCommand,
}

#[derive(Subcommand)]
enum GrdCliCommand {
    Install {
        owner: String,
        repo: String,
        tag: Option<String>,
        artifact_name: Option<String>,
    },
}

fn main() {
    println!("Hello, world!")
}
