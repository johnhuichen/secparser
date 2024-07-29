use colored::Colorize;
use core::panic;
use inquire::{validator::Validation, Confirm, Text};
use reqwest::{
    self,
    header::{ACCEPT_ENCODING, HOST, USER_AGENT},
    Url,
};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{copy, Write};
use std::path::{Path, PathBuf};
use std::{error::Error, thread::sleep, time::Duration};

#[derive(Debug, Serialize, Deserialize)]
struct DownloadConfig {
    user_agent: String,
}

impl DownloadConfig {
    pub fn new() -> Self {
        let filepath = "config/.download.toml";

        Self::create_file_if_missing(filepath);

        Self::parse_cfg_from_toml(filepath)
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

        if !contents.is_empty() {
            return toml::from_str(&contents)
                .unwrap_or_else(|e| panic!("Should parse config {filepath} {e}"));
        }

        let user_agent = Self::get_user_input("Enter an email address as user agent (see https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data for reasons)");
        let cfg = DownloadConfig { user_agent };
        let cfg_content = toml::to_string(&cfg).unwrap();
        let mut file = File::create(filepath).unwrap_or_else(|_| panic!());
        file.write_all(cfg_content.as_bytes())
            .unwrap_or_else(|_| panic!("Should write default config to empty config file"));

        cfg
    }

    fn get_user_input(msg: &str) -> String {
        let validator = |input: &str| {
            if input.chars().count() == 0 {
                Ok(Validation::Invalid("Empty input is not allowed".into()))
            } else {
                Ok(Validation::Valid)
            }
        };

        Text::new(msg)
            .with_validator(validator)
            .prompt()
            .unwrap_or_else(|e| panic!("Should get user input, {e}"))
    }
}

const TEMP_DIR: &str = "tmp";

pub async fn download(url: &str) -> Result<PathBuf, Box<dyn Error>> {
    let config = DownloadConfig::new();
    log::info!("Config loaded: {config:#?}");

    let filepath = get_filepath(url);

    if should_download(&filepath) {
        download_and_save(url, &filepath, &config).await?;
    }

    Ok(filepath)
}

fn get_filepath(url: &str) -> PathBuf {
    fs::create_dir_all(TEMP_DIR)
        .unwrap_or_else(|_| panic!("Should create directory: {}", TEMP_DIR));
    let parsed_url = Url::parse(url).unwrap_or_else(|_| panic!("Should parse url: {url}"));
    let filename = parsed_url
        .path_segments()
        .and_then(|segments| segments.last())
        .unwrap_or("tmp.bin");

    Path::new(TEMP_DIR).join(filename)
}

fn should_download(filepath: &Path) -> bool {
    if !filepath.exists() {
        return true;
    }

    let ans = Confirm::new(&format!(
        "File exists as {}, do you want to download it again?",
        filepath.to_string_lossy().bright_blue(),
    ))
    .with_help_message("Download if you are not sure of the file conntent")
    .prompt();

    ans.unwrap()
}

async fn download_and_save(
    url: &str,
    filepath: &PathBuf,
    config: &DownloadConfig,
) -> Result<(), Box<dyn Error>> {
    log::info!("Downloading {url} and save as {filepath:#?}");
    let client = reqwest::Client::new();
    // SEC allows max concurrent request of 10 requests/second.
    // This is the easiest way to ensure this restriction is followed
    // Further optimization may be needed if this becomes a performance bottleneck
    let sleep_duration = Duration::from_millis(100);
    sleep(sleep_duration);
    let response = client
        .get(url)
        .header(USER_AGENT, config.user_agent.to_string()) // see https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data for reasons
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
