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
use std::{error::Error, thread::sleep, time::Duration};

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

    pub async fn download(&mut self, url: &str) -> Result<PathBuf, Box<dyn Error>> {
        let filepath = self.get_filepath(url);

        if self.should_download(&filepath) {
            self.save_file(url, &filepath).await?;
        }

        Ok(filepath)
    }

    fn get_filepath(&self, url: &str) -> PathBuf {
        fs::create_dir_all(&self.temp_dir)
            .unwrap_or_else(|_| panic!("Should create directory: {}", self.temp_dir));
        let parsed_url = Url::parse(url).unwrap_or_else(|_| panic!("Should parse url: {url}"));
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

        let msg = format!(
            "A file with the same name already exists({}), do you want to download it again?",
            filepath.to_string_lossy().bright_blue()
        );
        let ans = Confirm::new(&msg)
            .with_default(false)
            .with_help_message("Download again may fetch new content")
            .prompt()
            .unwrap();

        if ans {
            log::info!("Will redownload all existing files");
        } else {
            log::info!("Will skip download for all existings files");
        }

        self.preferences.redownload = Some(ans);

        ans
    }

    async fn save_file(&self, url: &str, filepath: &PathBuf) -> Result<(), Box<dyn Error>> {
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
            .await?;
        let mut dest =
            File::create(filepath).unwrap_or_else(|_| panic!("Should create {filepath:?}"));
        let content = response.text().await?;
        copy(&mut content.as_bytes(), &mut dest)
            .unwrap_or_else(|_| panic!("Should copy download content to file"));
        Ok(())
    }
}
