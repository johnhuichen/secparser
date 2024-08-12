use std::fs::File;
use std::path::PathBuf;

use anyhow::{Error, Result};

use crate::downloader::{DownloadConfig, Downloader};

#[derive(Clone)]
pub struct DataSource {
    pub filepath: PathBuf,
}

impl DataSource {
    pub fn new(download_config: &DownloadConfig, url: &str) -> Result<Self> {
        let downloader = Downloader::new(download_config.clone());
        let filepath = downloader.download(url)?;

        Ok(Self { filepath })
    }

    pub fn validate(&self) -> Result<()> {
        if !self.filepath.exists() {
            return Err(Error::msg(format!("Should have {:?}", self.filepath)));
        }

        let file = File::open(&self.filepath)?;
        let file_size = file.metadata()?.len();

        if file_size == 0 {
            return Err(Error::msg(format!(
                "Should be non empty {:?}",
                self.filepath
            )));
        }

        Ok(())
    }
}
