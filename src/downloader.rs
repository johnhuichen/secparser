use colored::Colorize;
use core::panic;
use reqwest::header::{ACCEPT_ENCODING, HOST, USER_AGENT};
use reqwest::{self, Url};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::{copy, Write};
use std::path::{Path, PathBuf};

use crate::prompt;

#[derive(Debug, Serialize, Deserialize)]
struct DownloadConfig {
    #[serde(default = "default_user_agent")]
    user_agent: String,

    #[serde(default = "default_temp_dir")]
    temp_dir: String,
}

fn default_user_agent() -> String {
    "example@domain.com".to_string()
}

fn default_temp_dir() -> String {
    "tmp".to_string()
}

impl DownloadConfig {
    pub fn new() -> Self {
        let filepath = "config/download.local.toml";

        Self::create_file_if_missing(filepath);

        let cfg = Self::parse_cfg_from_toml(filepath);

        Self::panic_if_invalid_cfg(filepath, &cfg);

        cfg
    }

    fn create_file_if_missing(filepath: &str) {
        if !Path::new(filepath).exists() {
            File::create_new(filepath)
                .unwrap_or_else(|_| panic!("Should create new file {filepath}"));
        }
    }

    fn parse_cfg_from_toml(filepath: &str) -> Self {
        let contents = fs::read_to_string(filepath)
            .unwrap_or_else(|_| panic!("Should open config {filepath}"));

        let cfg: DownloadConfig = toml::from_str(&contents)
            .unwrap_or_else(|e| panic!("Should parse config {filepath} {e}"));

        if contents.is_empty() {
            let cfg_content = toml::to_string(&cfg).unwrap();
            let mut file = File::create(filepath).unwrap_or_else(|_| panic!());
            file.write_all(cfg_content.as_bytes())
                .unwrap_or_else(|_| panic!("Should write default config to empty config file"));
        }

        cfg
    }

    fn panic_if_invalid_cfg(filepath: &str, cfg: &Self) {
        if cfg.user_agent == default_user_agent() {
            panic!(
                "Default user agent {} is used. You have to update it in {}",
                cfg.user_agent.to_string().bright_red(),
                filepath.to_string().bright_blue()
            );
        }
    }
}

pub async fn download(url: &str) -> Result<PathBuf, Box<dyn Error>> {
    let config = DownloadConfig::new();
    let filepath = get_filepath(url, &config);

    if should_download(&filepath) {
        download_and_save(url, &filepath, &config).await?;
    }

    Ok(filepath)
}

fn get_filepath(url: &str, config: &DownloadConfig) -> PathBuf {
    fs::create_dir_all(&config.temp_dir)
        .unwrap_or_else(|_| panic!("Should create directory: {}", &config.temp_dir));
    let parsed_url = Url::parse(url).unwrap_or_else(|_| panic!("Should parse url: {url}"));
    let filename = parsed_url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("tmp.bin");

    Path::new(&config.temp_dir).join(filename)
}

fn should_download(filepath: &Path) -> bool {
    !filepath.exists()
        || prompt::confirm(&format!(
            "{} exists, do you want to download it again?",
            filepath.to_string_lossy().bright_blue()
        ))
}

async fn download_and_save(
    url: &str,
    filepath: &PathBuf,
    config: &DownloadConfig,
) -> Result<(), Box<dyn Error>> {
    log::info!("downloading {url} and save as {filepath:#?}");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, config.user_agent.to_string())
        .header(ACCEPT_ENCODING, "gzip,deflate".to_string())
        .header(HOST, "www.sec.gov".to_string())
        .send()
        .await?;
    let mut dest = File::create(filepath).unwrap_or_else(|_| panic!("Should create {filepath:?}"));
    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)
        .unwrap_or_else(|_| panic!("Should copy download content to file"));

    Ok(())
}
