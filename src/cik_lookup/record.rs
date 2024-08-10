use anyhow::Result;
use std::collections::HashMap;
use std::fs::{self};

use serde::Deserialize;

use crate::traits::{FileLines, FileReader};

use super::data_source::CikLookupDataSource;

#[derive(Debug)]
pub struct CikLookup {
    pub cik: usize,
    pub name: String,
    pub ticker: String,
    pub exchange: String,
}

type TickersExchangeFields = Vec<String>;
type TickersExchangeDataItem = (usize, Option<String>, Option<String>, Option<String>);

#[derive(Debug, Deserialize)]
struct TickersExchangeData {
    fields: TickersExchangeFields,
    data: Vec<TickersExchangeDataItem>,
}

pub struct CikLookupRecords {
    lines: FileLines,
    tickers_exchange: HashMap<usize, (Option<String>, Option<String>)>,
}

impl FileReader for CikLookupRecords {}

impl CikLookupRecords {
    pub fn new(datasource: CikLookupDataSource) -> Result<Self> {
        let lines = Self::get_lines(&datasource.lookup_filepath)?;

        let contents = fs::read_to_string(&datasource.tickers_exchange_filepath)?;
        let tickers_exchange: TickersExchangeData = serde_json::from_str(&contents)?;

        assert_eq!(
            tickers_exchange.fields,
            vec!("cik", "name", "ticker", "exchange")
        );

        let tickers_exchange = tickers_exchange
            .data
            .into_iter()
            .map(|item| (item.0, (item.2, item.3)))
            .collect::<HashMap<_, _>>();

        Ok(Self {
            lines,
            tickers_exchange,
        })
    }

    fn parse_one_record(&self, name: &str, cik: &str) -> CikLookup {
        let cik = cik
            .parse::<usize>()
            .unwrap_or_else(|e| panic!("Should parse cik: {e}"));
        let (ticker, exchange) = self.tickers_exchange.get(&cik).unwrap_or(&(None, None));
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
    }
}

impl Iterator for CikLookupRecords {
    type Item = CikLookup;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(line) => {
                let line = line.unwrap_or_else(|e| panic!("Should get line in cik lookup: {e}"));
                let line = &line[..line.len() - 1];

                line.rsplit_once(":")
                    .map(|(name, cik)| self.parse_one_record(name, cik))
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::downloader::DownloadConfigBuilder;
    use anyhow::Result;

    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        env_logger::init();

        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .download_dir("./download".to_string())
            .build()?;
        let data_source = CikLookupDataSource::new(&download_config)?;
        let records = CikLookupRecords::new(data_source)?;

        for r in records {
            if !r.ticker.is_empty() {
                assert_eq!(r.cik, 1084869);
                assert_eq!(r.name, "1 800 FLOWERS COM INC");
                assert_eq!(r.ticker, "FLWS");
                assert_eq!(r.exchange, "Nasdaq");
                break;
            }
        }

        Ok(())
    }
}
