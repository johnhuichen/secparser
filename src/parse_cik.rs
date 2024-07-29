use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::downloader::Downloader;
use crate::local_config::LocalConfig;

// See https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data
// Section "CIK" and "CIK, ticker, and exchange associations"

pub async fn download_and_parse() -> Result<(), Box<dyn Error>> {
    let mut downloader = Downloader::new();

    let url = "https://www.sec.gov/files/company_tickers_exchange.json";
    downloader.download(url).await?;

    let url = "https://www.sec.gov/data/company_tickers_mf.json";
    downloader.download(url).await?;

    parse_cik_lookup(&mut downloader).await?;

    Ok(())
}
async fn parse_cik_lookup(downloader: &mut Downloader) -> Result<(), Box<dyn Error>> {
    let local_config = LocalConfig::new();
    let url = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
    let filepath = downloader.download(url).await?;

    let path = Path::new(&local_config.out_dir).join("cik-lookup-doc.csv");
    let mut doc_writer = Writer::from_path(path)?;
    doc_writer.write_record(["column", "Description"])?;
    doc_writer.write_record([
        "cik",
        "EDGAR assigns to filers a unique numerical identifier, known as a Central Index Key",
    ])?;
    doc_writer.write_record(["company_name", "Company name"])?;
    doc_writer.flush()?;

    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);

    let count = reader.lines().count();
    let bar = ProgressBar::new(count as u64);
    bar.set_style(
        ProgressStyle::with_template("[{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    let path = Path::new(&local_config.out_dir).join("cik-lookup.csv");
    let mut doc_writer = Writer::from_path(path)?;
    doc_writer.write_record(["cik", "company_name"])?;
    let file = File::open(&filepath)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap_or_else(|_| panic!("Should get line from {filepath:?}"));
        let line = &line[..line.len() - 1];
        let (company_name, cik) = line.rsplit_once(":").unwrap();
        doc_writer.write_record([cik, company_name])?;

        bar.inc(1);
    }

    bar.finish();

    Ok(())
}
