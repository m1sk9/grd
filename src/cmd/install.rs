use clap::Parser;

#[derive(Debug, Parser)]
pub struct InstallCmd {
    owner: String,
    repo: String,
    artifact_name: Option<String>,
    tag: Option<String>,
}

impl InstallCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        println!("Running install command with args: {:?}", self);
        Ok(())
    }
}
