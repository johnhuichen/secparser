use csv::ReaderBuilder;
use derive_builder::Builder;
use serde::de::DeserializeOwned;
use snafu::{Location, Snafu};
use std::fs::File;
use std::io;
use std::io::BufReader;
use zip::ZipArchive;

use crate::data_source::DataSource;

#[derive(Debug, Snafu)]
pub enum ZipCsvRecordsError {
    #[snafu(display("IO error at {loc}"))]
    #[snafu(context(false))]
    IO {
        source: io::Error,
        #[snafu(implicit)]
        loc: Location,
    },

    #[snafu(display("Zip error at {loc}"))]
    #[snafu(context(false))]
    Zip {
        source: zip::result::ZipError,
        #[snafu(implicit)]
        loc: Location,
    },
}

#[derive(Clone, Debug, Builder)]
pub struct CsvConfig {
    #[builder(default = "true")]
    pub csv_flexible: bool,
    #[builder(default = "true")]
    pub csv_quoting: bool,
    #[builder(default = "false")]
    pub panic_on_error: bool,
}

pub struct ZipCsvRecords<T> {
    record_iter: std::vec::IntoIter<T>,
}

impl<T> ZipCsvRecords<T>
where
    T: DeserializeOwned,
{
    pub fn new(
        data_source: &DataSource,
        config: &CsvConfig,
        csv_filename: &str,
    ) -> Result<Self, ZipCsvRecordsError> {
        let file = File::open(&data_source.filepath)?;
        let mut archive = ZipArchive::new(file)?;

        let tag_file = archive.by_name(csv_filename)?;
        let reader = BufReader::new(tag_file);
        let reader = ReaderBuilder::new()
            .quoting(config.csv_quoting)
            .flexible(config.csv_flexible)
            .delimiter(b'\t')
            .from_reader(reader);
        let record_iter = reader
            .into_deserialize()
            .filter_map(|r| {
                if config.panic_on_error {
                    panic!("Should parse {:?}", data_source.filepath);
                } else {
                    r.ok()
                }
            })
            .collect::<Vec<T>>()
            .into_iter();

        Ok(Self { record_iter })
    }
}

impl<T> Iterator for ZipCsvRecords<T>
where
    T: DeserializeOwned,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.record_iter.next()
    }
}
