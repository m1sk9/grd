use clap::Parser;
use tracing::info;

use crate::assets::Assets;

#[derive(Debug, Parser)]
pub struct DownloadCmd {
    /// The GitHub owner of the repository.
    pub owner: String,

    /// The GitHub repository name.
    pub repo: String,

    /// The name of the artifact to install.
    pub artifact_name: String,

    /// The tag of the release to install. If not specified, the latest release will be installed.
    pub tag: String,
}

impl DownloadCmd {
    pub async fn run(self) -> anyhow::Result<()> {
        info!(
            "Installing {} from {}/{}@{}...",
            self.artifact_name, self.owner, self.repo, self.tag
        );

        // TODO: Implement URL parsing.
        let url = &format!(
            "https://github.com/{owner}/{repo}/releases/download/{tag}/{artifact_name}",
            owner = self.owner,
            repo = self.repo,
            tag = self.tag,
            artifact_name = self.artifact_name
        );
        let response = reqwest::get(url).await;

        println!("{}", url);
        match response {
            Ok(res) => {
                if !res.status().is_success() {
                    return Err(anyhow::anyhow!("Failed to download: {}", res.status()));
                }

                let metadata = Assets {
                    file_name: self.artifact_name,
                    byte: res.bytes().await?,
                };
                metadata.download_assets().await?;
                info!("Successfully installed.");
            }
            Err(err) => {
                return Err(anyhow::anyhow!("Failed to download: {}", err));
            }
        }
        Ok(())
    }
}
