use anyhow::Result;
use std::path::PathBuf;

use crate::downloader::{DownloadConfig, Downloader};
use crate::traits::DataSource;

pub struct CikLookupDataSource {
    pub lookup_filepath: PathBuf,
    pub tickers_exchange_filepath: PathBuf,
}

impl CikLookupDataSource {
    const LOOKUP_URL: &'static str = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
    const TICKERS_EXCHANGE_URL: &'static str =
        "https://www.sec.gov/files/company_tickers_exchange.json";

    pub fn new(download_config: &DownloadConfig) -> Result<Self> {
        let downloader = Downloader::new(download_config.clone());
        let lookup_filepath = downloader.download(Self::LOOKUP_URL)?;
        let tickers_exchange_filepath = downloader.download(Self::TICKERS_EXCHANGE_URL)?;

        Ok(Self {
            lookup_filepath,
            tickers_exchange_filepath,
        })
    }
}

impl DataSource for CikLookupDataSource {
    fn validate_cache(&self) -> Result<()> {
        let filepaths = vec![&self.lookup_filepath, &self.tickers_exchange_filepath];

        for filepath in filepaths {
            Self::validate_non_empty_file(filepath)?;
        }

        Ok(())
    }
}
