use std::io::Write;

use anyhow::{Context, Ok};
use bytes::Bytes;

#[derive(Debug)]
pub struct Assets {
    pub file_name: String,
    pub byte: Bytes,
}

impl Assets {
    pub async fn download_assets(&self) -> anyhow::Result<()> {
        let path = std::env::current_dir()?.join(&self.file_name);
        let mut assert_file = std::fs::File::create(path).context("Failed to create file")?;

        if let Err(why) = assert_file.write_all(&self.byte) {
            return Err(anyhow::anyhow!("Failed to write to file: {}", why));
        }
        Ok(())
    }
}
