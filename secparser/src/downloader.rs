use core::panic;
use reqwest::{
    self,
    header::{ACCEPT_ENCODING, HOST, USER_AGENT},
    Url,
};
use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};

pub struct Downloader {
    user_agent: String,
}

impl Downloader {
    const DOWNLOAD_DIR: &'static str = "download";

    pub fn new(user_agent: &str) -> Self {
        Downloader {
            user_agent: user_agent.to_owned(),
        }
    }

    pub async fn download(&mut self, url: &str) -> PathBuf {
        let filepath = self.get_filepath(url);

        if !filepath.exists() {
            self.save_file(url, &filepath).await;
        }

        filepath
    }

    fn get_filepath(&self, url: &str) -> PathBuf {
        fs::create_dir_all(Self::DOWNLOAD_DIR)
            .unwrap_or_else(|e| panic!("Should create directory {}: {e}", Self::DOWNLOAD_DIR));
        let parsed_url = Url::parse(url).unwrap_or_else(|e| panic!("Should parse url {url}: {e}"));
        let filename = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_else(|| panic!("Should parse filename from {url}"));

        Path::new(Self::DOWNLOAD_DIR).join(filename)
    }

    async fn save_file(&self, url: &str, filepath: &PathBuf) {
        let client = reqwest::Client::new();
        // See https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data
        // Section "Fair Access"
        let response = client
            .get(url)
            .header(USER_AGENT, self.user_agent.to_string())
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
            .unwrap_or_else(|e| panic!("Should parse response from {url}: {e}"));
        copy(&mut content.as_bytes(), &mut dest)
            .unwrap_or_else(|e| panic!("Should copy download content to file: {e}"));
    }
}
