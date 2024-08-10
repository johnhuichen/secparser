use anyhow::Result;
use colored::Colorize;
use core::panic;
use derive_builder::Builder;
use reqwest::{
    header::{ACCEPT_ENCODING, HOST, USER_AGENT},
    Url,
};
use retry::{
    delay::{jitter, Exponential},
    retry,
};
use std::io::copy;
use std::path::{Path, PathBuf};
use std::{
    fs::{self, File},
    time::Duration,
};

#[derive(Clone, Debug, Builder)]
pub struct DownloadConfig {
    pub user_agent: String,

    #[builder(default = "String::from(\"/tmp/secparser\")")]
    pub download_dir: String,

    #[builder(default = "true")]
    pub use_local_cache: bool,

    #[builder(default = "5")]
    pub retry_times: usize,

    #[builder(default = "1000")]
    pub retry_timeout: u64,
}

pub struct Downloader {
    config: DownloadConfig,
}

impl Downloader {
    pub fn new(config: DownloadConfig) -> Self {
        Downloader { config }
    }

    pub fn download(&self, url: &str) -> Result<PathBuf> {
        let filepath = self.get_filepath(url)?;

        if !self.config.use_local_cache
            || !filepath.exists()
            || File::open(&filepath)?.metadata()?.len() == 0
        {
            let retry_iter = Exponential::from_millis(self.config.retry_timeout)
                .map(jitter)
                .take(self.config.retry_times);

            retry(retry_iter, || self.download_and_save(url, &filepath))
                .map_err(|e| anyhow::anyhow!(e))?;
        } else {
            log::debug!("{}", format!("Skip downloading {url}").bright_blue());
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

    fn download_and_save(&self, url: &str, filepath: &PathBuf) -> Result<()> {
        log::debug!("{}", format!("Downloading {url}").bright_magenta());

        let client = reqwest::blocking::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(3))
            .build()?;
        // See https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data
        // Section "Fair Access"
        let response = client
            .get(url)
            .header(USER_AGENT, self.config.user_agent.to_string())
            .header(ACCEPT_ENCODING, "gzip,deflate".to_string())
            .header(HOST, "www.sec.gov".to_string())
            .send()
            .map_err(|e| {
                log::debug!("{}", format!("Failed to download {url}").bright_red());
                e
            })?;
        let mut dest = File::create(filepath)?;
        let content = response.bytes()?;
        copy(&mut content.as_ref(), &mut dest)?;

        log::debug!(
            "{}",
            format!("Downloaded {url} to {filepath:?}").bright_green()
        );
        Ok(())
    }
}
