use colored::Colorize;
use core::panic;
use inquire::Confirm;
use reqwest::{
    self,
    header::{ACCEPT_ENCODING, HOST, USER_AGENT},
    Url,
};
use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};
use std::{thread::sleep, time::Duration};

use crate::local_config::LocalConfig;

pub struct DownloaderPreferences {
    redownload: Option<bool>,
}

pub struct Downloader {
    preferences: DownloaderPreferences,
    user_agent: String,
    temp_dir: String,
}

impl Downloader {
    pub fn new() -> Self {
        let preferences = DownloaderPreferences { redownload: None };
        let config = LocalConfig::new();
        Downloader {
            preferences,
            user_agent: config.user_agent,
            temp_dir: config.temp_dir,
        }
    }

    pub async fn download(&mut self, url: &str) -> PathBuf {
        let filepath = self.get_filepath(url);

        if self.should_download(&filepath) {
            self.save_file(url, &filepath).await;
        }

        filepath
    }

    fn get_filepath(&self, url: &str) -> PathBuf {
        fs::create_dir_all(&self.temp_dir)
            .unwrap_or_else(|e| panic!("Should create directory {}: {e}", self.temp_dir));
        let parsed_url = Url::parse(url).unwrap_or_else(|e| panic!("Should parse url {url}: {e}"));
        let filename = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("tmp.bin");

        Path::new(&self.temp_dir).join(filename)
    }

    fn should_download(&mut self, filepath: &Path) -> bool {
        if !filepath.exists() {
            return true;
        }

        if let Some(ans) = self.preferences.redownload {
            return ans;
        }

        let msg = format!("{filepath:?} already exists, do you want to download it again?");
        let msg = format!("{}", msg.bright_yellow());
        let ans = Confirm::new(&msg)
            .with_default(false)
            .with_help_message("Default to skip download")
            .prompt()
            .unwrap();

        if ans {
            log::info!("{}", "Will redownload all existing files".bright_yellow());
        } else {
            log::info!(
                "{}",
                "Will skip download for all existings files".bright_purple()
            );
        }

        self.preferences.redownload = Some(ans);

        ans
    }

    async fn save_file(&self, url: &str, filepath: &PathBuf) {
        let client = reqwest::Client::new();
        // SEC allows max concurrent request of 10 requests/second.
        // This is the easiest way to ensure this restriction is followed
        // Further optimization may be needed if this becomes a performance bottleneck
        let sleep_duration = Duration::from_millis(100);
        sleep(sleep_duration);
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
