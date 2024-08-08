use anyhow::Result;
use colored::Colorize;
use core::panic;
use derive_builder::Builder;
use reqwest::{
    self,
    header::{ACCEPT_ENCODING, HOST, USER_AGENT},
    Url,
};
use std::io::copy;
use std::path::{Path, PathBuf};
use std::{
    fs::{self, File},
    thread::sleep,
    time::Duration,
};

use crate::macros::retry;

#[derive(Clone, Debug, Builder)]
pub struct DownloadConfig {
    pub user_agent: String,

    #[builder(default = "String::from(\"/tmp/secparser\")")]
    pub download_dir: String,

    #[builder(default = "true")]
    pub use_local_cache: bool,
}

pub struct Downloader {
    config: DownloadConfig,
}

impl Downloader {
    pub fn new(config: DownloadConfig) -> Self {
        Downloader { config }
    }

    pub async fn download(&self, url: &str) -> Result<PathBuf> {
        let filepath = self.get_filepath(url)?;

        if !self.config.use_local_cache || !filepath.exists() {
            self.download_and_save(url, &filepath).await?;
        } else {
            log::debug!("Skip downloading {url}");
        }

        Ok(filepath)
    }

    pub fn get_filepath(&self, url: &str) -> Result<PathBuf> {
        fs::create_dir_all(&self.config.download_dir)?;
        let parsed_url = Url::parse(url)?;
        let filename = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_else(|| panic!("Should parse filename from {url}"));

        Ok(Path::new(&self.config.download_dir).join(filename))
    }

    async fn download_and_save(&self, url: &str, filepath: &PathBuf) -> Result<()> {
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(3))
            .build()?;

        retry! {
            async {
                log::debug!("Downloading {url}");
                // See https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data
                // Section "Fair Access"
                sleep(Duration::from_millis(100));
                let response = client
                    .get(url)
                    .header(USER_AGENT, self.config.user_agent.to_string())
                    .header(ACCEPT_ENCODING, "gzip,deflate".to_string())
                    .header(HOST, "www.sec.gov".to_string())
                    .send()
                    .await?;
                let mut dest = File::create(filepath)?;
                let content = response.text().await?;
                copy(&mut content.as_bytes(), &mut dest)?;

                log::debug!(
                    "{}",
                    format!("Downloaded {url} to {filepath:?}").bright_green()
                );
                Ok(())
            }.await, 3, 1000
        }
    }
}
