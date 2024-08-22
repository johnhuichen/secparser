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
use snafu::{Location, ResultExt, Snafu};
use std::io;
use std::path::{Path, PathBuf};
use std::{
    fs::{self, File},
    time::Duration,
};

#[derive(Debug, Snafu)]
pub enum DownloaderError {
    #[snafu(display("IO error at {loc}"))]
    #[snafu(context(false))]
    IO {
        source: io::Error,
        #[snafu(implicit)]
        loc: Location,
    },

    #[snafu(display("Failed to get file path for url {url}"))]
    GetFilePath {
        source: GetFilePathError,
        url: String,
    },

    #[snafu(display("Failed to download url {url}"))]
    DownloadAndSave {
        source: retry::Error<DownloadAndSaveError>,
        url: String,
    },
}

#[derive(Debug, Snafu)]
pub enum GetFilePathError {
    #[snafu(display("Could not create dir {path}"))]
    CreateDir { source: io::Error, path: String },

    #[snafu(display("Could not parse {url}"))]
    ParseUrl {
        source: url::ParseError,
        url: String,
    },
}

#[derive(Debug, Snafu)]
pub enum DownloadAndSaveError {
    #[snafu(display("Failed to build client"))]
    ClientBuilder { source: reqwest::Error },

    #[snafu(display("Failed to download {url}"))]
    Download { source: reqwest::Error, url: String },

    #[snafu(display("IO error at {loc}"))]
    #[snafu(context(false))]
    IO {
        source: io::Error,
        #[snafu(implicit)]
        loc: Location,
    },

    #[snafu(display("Failed to read response to bytes {url}"))]
    ResponseBytes { source: reqwest::Error, url: String },
}

#[derive(Clone, Debug, Builder)]
pub struct DownloadConfig {
    pub user_agent: String,

    #[builder(default = "String::from(\"./download\")")]
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

    pub fn download(&self, url: &str) -> Result<PathBuf, DownloaderError> {
        let filepath = self.get_filepath(url).context(GetFilePathSnafu { url })?;

        if !self.config.use_local_cache
            || !filepath.exists()
            || File::open(&filepath)?.metadata()?.len() == 0
        {
            let retry_iter = Exponential::from_millis(self.config.retry_timeout)
                .map(jitter)
                .take(self.config.retry_times);

            retry(retry_iter, || self.download_and_save(url, &filepath))
                .context(DownloadAndSaveSnafu { url })?;
        } else {
            log::debug!("{}", format!("Skip downloading {url}").bright_blue());
        }

        Ok(filepath)
    }

    pub fn get_filepath(&self, url: &str) -> Result<PathBuf, GetFilePathError> {
        fs::create_dir_all(&self.config.download_dir).context(CreateDirSnafu {
            path: self.config.download_dir.to_string(),
        })?;
        let parsed_url = Url::parse(url).context(ParseUrlSnafu { url })?;
        let filename = parsed_url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or_else(|| panic!("Should parse filename from {url}"));

        Ok(Path::new(&self.config.download_dir).join(filename))
    }

    fn download_and_save(&self, url: &str, filepath: &PathBuf) -> Result<(), DownloadAndSaveError> {
        log::debug!("{}", format!("Downloading {url}").bright_magenta());

        let client = reqwest::blocking::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(3))
            .build()
            .context(ClientBuilderSnafu)?;
        // See https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data
        // Section "Fair Access"
        let response = client
            .get(url)
            .header(USER_AGENT, self.config.user_agent.to_string())
            .header(ACCEPT_ENCODING, "gzip,deflate")
            .header(HOST, "www.sec.gov")
            .send()
            .context(DownloadSnafu { url })?;
        let mut dest = File::create(filepath)?;
        let content = response.bytes().context(ResponseBytesSnafu { url })?;
        io::copy(&mut content.as_ref(), &mut dest)?;

        log::debug!(
            "{}",
            format!("Downloaded {url} to {filepath:?}").bright_green()
        );
        Ok(())
    }
}
