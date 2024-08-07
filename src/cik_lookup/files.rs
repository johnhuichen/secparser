use anyhow::Result;
use std::path::PathBuf;

use crate::downloader::{DownloadConfig, Downloader};

pub struct CikLookupFiles {
    pub lookup_filepath: PathBuf,
    pub tickers_exchange_filepath: PathBuf,
}

impl CikLookupFiles {
    const LOOKUP_URL: &'static str = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
    const TICKERS_EXCHANGE_URL: &'static str =
        "https://www.sec.gov/files/company_tickers_exchange.json";

    pub fn get_local_cache(download_config: DownloadConfig) -> Result<Self> {
        let downloader = Downloader::new(download_config);
        let lookup_filepath = downloader.get_filepath(Self::LOOKUP_URL)?;
        let tickers_exchange_filepath = downloader.get_filepath(Self::TICKERS_EXCHANGE_URL)?;

        Ok(Self {
            lookup_filepath,
            tickers_exchange_filepath,
        })
    }

    pub async fn download(download_config: DownloadConfig) -> Result<Self> {
        let downloader = Downloader::new(download_config);
        let lookup_filepath = downloader.download(Self::LOOKUP_URL).await?;
        let tickers_exchange_filepath = downloader.download(Self::TICKERS_EXCHANGE_URL).await?;

        Ok(Self {
            lookup_filepath,
            tickers_exchange_filepath,
        })
    }
}
