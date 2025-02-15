use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use snafu::{Location, ResultExt, Snafu};

use crate::data_source::DataSourceError;
use crate::downloader::DownloadConfig;
use crate::traits::{FileLines, FileReader};

use super::data_source::CikLookupDataSources;

#[derive(Debug, Snafu)]
pub enum CikLookupRecordsError {
    #[snafu(display("Failed to get data source"))]
    DataSource { source: DataSourceError },

    #[snafu(display("IO error at {loc}"))]
    #[snafu(context(false))]
    IO {
        source: io::Error,
        #[snafu(implicit)]
        loc: Location,
    },

    #[snafu(display("Failed to deserialize {filepath:?}"))]
    Deserialize {
        source: serde_json::Error,
        filepath: PathBuf,
    },
}

#[derive(Debug, Serialize)]
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
    pub count: usize,
    lines: FileLines,
    tickers_exchange: HashMap<usize, (Option<String>, Option<String>)>,
}

impl FileReader for CikLookupRecords {}

impl CikLookupRecords {
    pub fn new(download_config: &DownloadConfig) -> Result<Self, CikLookupRecordsError> {
        let data_source = CikLookupDataSources::new(download_config).context(DataSourceSnafu)?;
        let lines = Self::get_lines(&data_source.lookup_ds.filepath)?;
        let count = Self::get_lines(&data_source.lookup_ds.filepath)?.count();

        let file = File::open(&data_source.tickers_exchange_ds.filepath)?;
        let reader = BufReader::new(file);
        let tickers_exchange: TickersExchangeData =
            serde_json::from_reader(reader).context(DeserializeSnafu {
                filepath: &data_source.tickers_exchange_ds.filepath,
            })?;

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
            count,
            tickers_exchange,
        })
    }

    fn parse_one_record(&self, name: &str, cik: &str) -> CikLookup {
        let cik = cik
            .parse::<usize>()
            .unwrap_or_else(|e| panic!("Should parse cik: {e}"));
        let (ticker, exchange) = self.tickers_exchange.get(&cik).unwrap_or(&(None, None));
        let name = name.to_string();
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
            name,
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
                let line =
                    line.unwrap_or_else(|e| panic!("Should get line in cik-lookup-data.txt: {e}"));
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
    use snafu::{ResultExt, Whatever};

    use crate::downloader::DownloadConfigBuilder;

    use super::*;

    #[test]
    fn it_parses_cik_lookup() -> Result<(), Whatever> {
        env_logger::builder()
            .is_test(true)
            .try_init()
            .unwrap_or_default();

        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .download_dir("./download".to_string())
            .build()
            .whatever_context("Failed to build config")?;
        let records =
            CikLookupRecords::new(&download_config).whatever_context("Failed to create records")?;

        for r in records {
            log::debug!("{r:?}");
        }

        Ok(())
    }
}
