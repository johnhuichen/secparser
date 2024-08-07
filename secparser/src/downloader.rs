use core::panic;
use derive_builder::Builder;
use reqwest::{
    self,
    header::{ACCEPT_ENCODING, HOST, USER_AGENT},
    Url,
};
use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Builder)]
pub struct DownloadConfig {
    pub user_agent: String,

    #[builder(default = "String::from(\"/tmp/secparser\")")]
    pub download_dir: String,

    #[builder(default = "false")]
    pub use_local_cache: bool,
}

pub struct Downloader {
    config: DownloadConfig,
}

impl Downloader {
    pub fn new(config: DownloadConfig) -> Self {
        Downloader { config }
    }

    pub async fn download(&mut self, url: &str) -> PathBuf {
        let filepath = self.get_filepath(url);

        if !self.config.use_local_cache || !filepath.exists() {
            self.save_file(url, &filepath).await;
        }

        filepath
    }

    fn get_filepath(&self, url: &str) -> PathBuf {
        fs::create_dir_all(&self.config.download_dir)
            .unwrap_or_else(|e| panic!("Should create {}: {e}", self.config.download_dir));
        let parsed_url = Url::parse(url).unwrap_or_else(|e| panic!("Should parse {url}: {e}"));
        let filename = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_else(|| panic!("Should parse filename from {url}"));

        Path::new(&self.config.download_dir).join(filename)
    }

    async fn save_file(&self, url: &str, filepath: &PathBuf) {
        let client = reqwest::Client::new();
        // See https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data
        // Section "Fair Access"
        let response = client
            .get(url)
            .header(USER_AGENT, self.config.user_agent.to_string())
            .header(ACCEPT_ENCODING, "gzip,deflate".to_string())
            .header(HOST, "www.sec.gov".to_string())
            .send()
            .await
            .unwrap_or_else(|e| panic!("Should download {url}: {e}"));
        let mut dest =
            File::create(filepath).unwrap_or_else(|e| panic!("Should create {filepath:?}: {e}"));
        let content = response
            .text()
            .await
            .unwrap_or_else(|e| panic!("Should parse response body from {url}: {e}"));
        copy(&mut content.as_bytes(), &mut dest)
            .unwrap_or_else(|e| panic!("Should copy downloaded file {url} to {filepath:?}: {e}"));
    }
}
