use crate::downloader::Downloader;
use crate::local_config::LocalConfig;
use colored::Colorize;

mod parse_cik_lookup;
mod parse_company_tickers_exchange;

// See https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data
// Section "CIK" and "CIK, ticker, and exchange associations"

pub async fn parse() {
    let mut downloader = Downloader::new();
    let local_config = LocalConfig::new();

    parse_cik_lookup::parse(&mut downloader, &local_config).await;
    parse_company_tickers_exchange::parse(&mut downloader, &local_config).await;

    let msg = format!(
        "CIK lookup parsing is completed. Find parsed csv files in {} folder",
        local_config.out_dir
    );
    log::info!("{}", msg.bright_yellow());
}
