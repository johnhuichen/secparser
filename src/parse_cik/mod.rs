use crate::downloader::Downloader;

mod parse_cik_lookup;
mod parse_company_tickers_exchange;

// See https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data
// Section "CIK" and "CIK, ticker, and exchange associations"

pub async fn download_and_parse() {
    let mut downloader = Downloader::new();

    parse_cik_lookup::parse(&mut downloader).await;
    parse_company_tickers_exchange::parse(&mut downloader).await;

    // let url = "https://www.sec.gov/data/company_tickers_mf.json";
    // downloader.download(url).await?;
}
