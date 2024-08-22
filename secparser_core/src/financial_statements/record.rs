use std::vec;

use crate::data_source::{DataSource, DataSourceError};
use crate::downloader::DownloadConfig;
use crate::financial_statements::data_source::FsDataSources;
use crate::zip_csv_records::{CsvConfig, ZipCsvRecords, ZipCsvRecordsError};
use serde::de::DeserializeOwned;
use snafu::{ResultExt, Snafu};

pub type DataSourceIter = vec::IntoIter<DataSource>;

pub struct FsRecords<T>
where
    T: DeserializeOwned,
{
    pub config: CsvConfig,
    pub data_source_iter: DataSourceIter,
    pub maybe_records: Option<ZipCsvRecords<T>>,
    pub csv_filename: String,
}

#[derive(Debug, Snafu)]
pub enum FsRecordsError {
    #[snafu(display("Failed to process csv"))]
    ZipCsv { source: ZipCsvRecordsError },

    #[snafu(display("Failed to get data source"))]
    DataSource { source: DataSourceError },
}

impl<T> FsRecords<T>
where
    T: DeserializeOwned,
{
    pub fn new(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
        csv_filename: &str,
    ) -> Result<Self, FsRecordsError> {
        let data_sources =
            FsDataSources::new(download_config, from_year).context(DataSourceSnafu)?;
        let data_source_iter = data_sources.vec.clone().into_iter();

        let mut result = Self {
            config,
            data_source_iter,
            maybe_records: None,
            csv_filename: csv_filename.to_string(),
        };

        result.get_maybe_record_iter().context(ZipCsvSnafu)?;

        Ok(result)
    }

    fn get_maybe_record_iter(&mut self) -> Result<(), ZipCsvRecordsError> {
        match self.data_source_iter.next() {
            Some(data_source) => {
                let records: ZipCsvRecords<T> =
                    ZipCsvRecords::new(&data_source, &self.config, &self.csv_filename)?;

                self.maybe_records = Some(records);

                Ok(())
            }
            None => {
                self.maybe_records = None;
                Ok(())
            }
        }
    }
}

impl<T> Iterator for FsRecords<T>
where
    T: DeserializeOwned,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.maybe_records {
                Some(record_iter) => match record_iter.next() {
                    Some(v) => return Some(v),
                    None => {
                        self.get_maybe_record_iter()
                            .unwrap_or_else(|e| panic!("Should get record iterator: {e}"));
                    }
                },
                None => return None,
            }
        }
    }
}
