use crate::data_source::{DataSource, DataSourceError};
use crate::downloader::DownloadConfig;

pub struct CikLookupDataSources {
    pub lookup_ds: DataSource,
    pub tickers_exchange_ds: DataSource,
}

impl CikLookupDataSources {
    const LOOKUP_URL: &'static str = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
    const TICKERS_EXCHANGE_URL: &'static str =
        "https://www.sec.gov/files/company_tickers_exchange.json";

    pub fn new(download_config: &DownloadConfig) -> Result<Self, DataSourceError> {
        let lookup_ds = DataSource::new(download_config, Self::LOOKUP_URL)?;
        let tickers_exchange_ds = DataSource::new(download_config, Self::TICKERS_EXCHANGE_URL)?;

        Ok(Self {
            lookup_ds,
            tickers_exchange_ds,
        })
    }
}
