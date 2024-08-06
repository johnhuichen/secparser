use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::downloader::Downloader;

type FileLines = Lines<BufReader<File>>;

#[derive(Debug)]
pub struct CikLookup {
    pub cik: usize,
    pub name: String,
    pub ticker: String,
    pub exchange: String,
}

pub struct CikLookupRecords {
    pub count: usize,

    lines: FileLines,
    tickers_exchange: HashMap<usize, (Option<String>, Option<String>)>,
}

type TickersExchangeFields = Vec<String>;
type TickersExchangeDataItem = (usize, Option<String>, Option<String>, Option<String>);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TickersExchangeData {
    fields: TickersExchangeFields,
    data: Vec<TickersExchangeDataItem>,
}

impl CikLookupRecords {
    pub async fn new(user_agent: &str, download_dir: &str) -> Self {
        let mut downloader = Downloader::new(user_agent, download_dir);
        let url = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
        let filepath = downloader.download(url).await;

        let lines = Self::get_lines(&filepath);
        let count = lines.count();
        let lines = Self::get_lines(&filepath);

        let url = "https://www.sec.gov/files/company_tickers_exchange.json";
        let filepath = downloader.download(url).await;

        let contents = fs::read_to_string(&filepath)
            .unwrap_or_else(|e| panic!("Should open {filepath:?}: {e}"));

        let tickers_exchange: TickersExchangeData = serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Should parse {filepath:?}: {e}"));

        assert_eq!(
            tickers_exchange.fields,
            vec!("cik", "name", "ticker", "exchange")
        );

        let tickers_exchange = tickers_exchange
            .data
            .into_iter()
            .map(|item| (item.0, (item.2, item.3)))
            .collect::<HashMap<_, _>>();

        Self {
            count,
            lines,
            tickers_exchange,
        }
    }

    fn get_lines(filepath: &PathBuf) -> FileLines {
        let file = File::open(filepath).unwrap_or_else(|e| panic!("Should open {filepath:?}: {e}"));
        let reader = BufReader::new(file);

        reader.lines()
    }
}

impl Iterator for CikLookupRecords {
    type Item = CikLookup;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(line) => {
                let line = line.unwrap_or_else(|e| panic!("Should get line in cik lookup: {e}"));
                let line = &line[..line.len() - 1];

                line.rsplit_once(":").map(|(name, cik)| {
                    let cik = cik
                        .parse::<usize>()
                        .unwrap_or_else(|e| panic!("Should parse cik: {e}"));
                    let (ticker, exchange) =
                        self.tickers_exchange.get(&cik).unwrap_or(&(None, None));
                    let ticker = match ticker {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    };
                    let exchange = match exchange {
                        Some(v) => v.to_string(),
                        None => "".to_string(),
                    };

                    CikLookup {
                        cik,
                        name: name.to_string(),
                        ticker,
                        exchange,
                    }
                })
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let records = CikLookupRecords::new("example@secparser.com", "/tmp/secparser").await;

        assert_eq!(records.count, 954999);

        for r in records {
            if !r.ticker.is_empty() {
                assert_eq!(r.cik, 1084869);
                assert_eq!(r.name, "1 800 FLOWERS COM INC");
                assert_eq!(r.ticker, "FLWS");
                assert_eq!(r.exchange, "Nasdaq");
                break;
            }
        }
    }
}
