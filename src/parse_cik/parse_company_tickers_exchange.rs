use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::csv_writer::{write_csv, CsvRecords};
use crate::downloader::Downloader;
use crate::local_config::LocalConfig;

type Fields = Vec<String>;
type DataItem = (usize, Option<String>, Option<String>, Option<String>);
type Data = Vec<DataItem>;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CompanyTickersExchangeData {
    fields: Fields,
    data: Data,
}

struct CompanyTickersExchangeIterator {
    iter: std::vec::IntoIter<DataItem>,
}

impl CompanyTickersExchangeIterator {
    fn new(data: CompanyTickersExchangeData) -> Self {
        let iter = data.data.into_iter();
        Self { iter }
    }
}

impl Iterator for CompanyTickersExchangeIterator {
    type Item = [String; 4];

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(line) => Some([
                line.0.to_string(),
                line.1.unwrap_or_default(),
                line.2.unwrap_or_default(),
                line.3.unwrap_or_default(),
            ]),
            None => None,
        }
    }
}

struct CompanyTickersExchangeRecords {
    data: CompanyTickersExchangeData,
}

impl CompanyTickersExchangeRecords {
    fn new(filepath: PathBuf) -> Self {
        let contents = fs::read_to_string(&filepath)
            .unwrap_or_else(|e| panic!("Should open config {filepath:?}: {e}"));

        let data: CompanyTickersExchangeData = serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Should parse json from {filepath:?}: {e}"));

        Self { data }
    }
}

impl CsvRecords for CompanyTickersExchangeRecords {
    type LineIter = [String; 4];
    type RecordIter = CompanyTickersExchangeIterator;

    fn get_headers(&self) -> Vec<String> {
        vec![
            "cik".to_string(),
            "name".to_string(),
            "ticker".to_string(),
            "exchange".to_string(),
        ]
    }

    fn get_count(&self) -> u64 {
        self.data.data.len() as u64
    }

    fn get_iter(&self) -> Self::RecordIter {
        CompanyTickersExchangeIterator::new(self.data.clone())
    }
}

impl Iterator for CompanyTickersExchangeRecords {
    type Item = DataItem;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub async fn parse(downloader: &mut Downloader, local_config: &LocalConfig) {
    log::info!("Downloading company tickers exchange data");
    let url = "https://www.sec.gov/files/company_tickers_exchange.json";
    let filepath = downloader.download(url).await;

    log::info!("Parsing company tickers exchange data");
    let records = CompanyTickersExchangeRecords::new(filepath);
    let csv_path = Path::new(&local_config.out_dir).join("company_tickers_exchange.csv");
    write_csv(&csv_path, records);
}
