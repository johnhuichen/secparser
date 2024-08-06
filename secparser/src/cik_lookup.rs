use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Lines};

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
    iter: FileLines,
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
    pub async fn new(user_agent: &str) -> Self {
        let mut downloader = Downloader::new(user_agent);
        let url = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
        let filepath = downloader.download(url).await;

        let file =
            File::open(&filepath).unwrap_or_else(|e| panic!("Should open {filepath:?}: {e}"));
        let reader = BufReader::new(file);

        let lines = reader.lines();

        let url = "https://www.sec.gov/files/company_tickers_exchange.json";
        let filepath = downloader.download(url).await;

        let contents = fs::read_to_string(&filepath)
            .unwrap_or_else(|e| panic!("Should open {filepath:?}: {e}"));

        let tickers_exchange: TickersExchangeData = serde_json::from_str(&contents)
            .unwrap_or_else(|e| panic!("Should parse {filepath:?}: {e}"));

        let tickers_exchange = tickers_exchange
            .data
            .into_iter()
            .map(|item| (item.0, (item.2, item.3)))
            .collect::<HashMap<_, _>>();

        Self {
            iter: lines,
            tickers_exchange,
        }
    }
}

impl Iterator for CikLookupRecords {
    type Item = CikLookup;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
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
