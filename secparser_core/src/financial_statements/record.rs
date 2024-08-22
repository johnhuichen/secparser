use std::fmt::Debug;
use std::vec;

use crate::data_source::{DataSource, DataSourceError};
use crate::downloader::{DownloadConfig, DownloadConfigBuilder};
use crate::financial_statements::data_source::FsDataSources;
use crate::zip_csv_records::{CsvConfig, CsvConfigBuilder, ZipCsvRecords, ZipCsvRecordsError};
use serde::de::DeserializeOwned;
use snafu::{ResultExt, Snafu, Whatever};

#[derive(Debug, Snafu)]
pub enum FsRecordsError {
    #[snafu(display("Failed to process csv"))]
    ZipCsv { source: ZipCsvRecordsError },

    #[snafu(display("Failed to get data source"))]
    DataSource { source: DataSourceError },
}

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
                log::info!("Processing {:?}", data_source.filepath);
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

pub trait FsService<T>
where
    T: DeserializeOwned,
{
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<T>, FsRecordsError>;
}

pub fn test_fs_records<R, T>() -> Result<(), Whatever>
where
    R: FsService<T>,
    T: DeserializeOwned + Debug,
{
    env_logger::builder()
        // .is_test(true)
        .try_init()
        .unwrap_or_default();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()
        .whatever_context("Failed to build config")?;
    let record_config = CsvConfigBuilder::default()
        .panic_on_error(true)
        .build()
        .whatever_context("Failed to build csv config")?;
    let from_year = 2024;

    let records = R::get_records(&download_config, record_config, from_year)
        .whatever_context("Failed to parse records")?;
    for record in records {
        log::info!("{:?}", record);
    }

    Ok(())
}
