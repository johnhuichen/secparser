use aliasable::boxed::AliasableBox;
use csv::{DeserializeRecordsIntoIter, ReaderBuilder};
use derive_builder::Builder;
use serde::de::DeserializeOwned;
use snafu::{Location, Snafu};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::iter::FilterMap;
use zip::read::ZipFile;
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

type FilterMapRecord<T> = fn(record: Result<T, csv::Error>) -> Option<T>;
type RecordIter<'archive, T> =
    FilterMap<DeserializeRecordsIntoIter<BufReader<ZipFile<'archive>>, T>, FilterMapRecord<T>>;

pub struct ZipCsvRecords<T>
where
    T: DeserializeOwned,
{
    record_iter: RecordIter<'static, T>,
    _archive: AliasableBox<ZipArchive<File>>,
}

impl<T> ZipCsvRecords<T>
where
    T: DeserializeOwned,
{
    pub fn new(
        data_source: &DataSource,
        config: &CsvConfig,
        csv_file: &str,
    ) -> Result<Self, ZipCsvRecordsError> {
        let zip_file = File::open(&data_source.filepath)?;
        let archive = ZipArchive::new(zip_file)?;
        let mut archive = AliasableBox::from_unique(Box::new(archive));

        let file = archive.by_name(csv_file)?;
        let reader = BufReader::new(file);
        let reader = ReaderBuilder::new()
            .quoting(config.csv_quoting)
            .flexible(config.csv_flexible)
            .delimiter(b'\t')
            .from_reader(reader);

        fn panic_on_error<T>(maybe_record: Result<T, csv::Error>) -> Option<T> {
            Some(maybe_record.unwrap_or_else(|e| panic!("Should parse {e}")))
        }
        let record_iter: RecordIter<T> = reader.into_deserialize().filter_map(panic_on_error);
        let record_iter: RecordIter<T> = unsafe { std::mem::transmute(record_iter) };

        Ok(Self {
            record_iter,
            _archive: archive,
        })
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
