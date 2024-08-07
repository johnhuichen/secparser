use anyhow::Result;
use std::collections::HashMap;
use std::fs::{self};

use serde::{Deserialize, Serialize};

use crate::traits::{FileLines, FileReader};

use super::files::CikLookupFiles;

#[derive(Debug)]
pub struct CikLookup {
    pub cik: usize,
    pub name: String,
    pub ticker: String,
    pub exchange: String,
}

type TickersExchangeFields = Vec<String>;
type TickersExchangeDataItem = (usize, Option<String>, Option<String>, Option<String>);

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TickersExchangeData {
    fields: TickersExchangeFields,
    data: Vec<TickersExchangeDataItem>,
}

pub struct CikLookupRecords {
    pub count: usize,

    lines: FileLines,
    tickers_exchange: HashMap<usize, (Option<String>, Option<String>)>,
}

impl FileReader for CikLookupRecords {}

impl CikLookupRecords {
    pub fn new(files: CikLookupFiles) -> Result<Self> {
        let count = Self::get_lines_count(&files.lookup_filepath)?;
        let lines = Self::get_lines(&files.lookup_filepath)?;

        let contents = fs::read_to_string(&files.tickers_exchange_filepath)?;

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
            count,
            lines,
            tickers_exchange,
        })
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

                    Self::Item {
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
    use crate::downloader::DownloadConfigBuilder;
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn it_works() -> Result<()> {
        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .build()?;
        let files = CikLookupFiles::download(download_config).await?;
        let records = CikLookupRecords::new(files)?;

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

    #[tokio::test]
    async fn it_parses_cached_files() -> Result<()> {
        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .build()?;
        let files = CikLookupFiles::get_local_cache(download_config)?;
        let records = CikLookupRecords::new(files)?;

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
