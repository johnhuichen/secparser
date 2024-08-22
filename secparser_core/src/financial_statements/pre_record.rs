use serde::Deserialize;

use crate::downloader::DownloadConfig;
use crate::zip_csv_records::CsvConfig;

use super::record::{FsRecords, FsRecordsError, FsService};

#[derive(Debug, Deserialize)]
pub struct FsPre {
    pub adsh: String,
    pub report: Option<u16>,
    pub line: Option<u32>,
    pub stmt: String,
    pub inpth: Option<u8>,
    pub tag: String,
    pub version: String,
    pub prole: String,
    pub plabel: String,
    pub negating: Option<u8>,
}

pub struct FsPreService {}

impl FsService<FsPre> for FsPreService {
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<FsPre>, FsRecordsError> {
        FsRecords::new(download_config, config, from_year, "pre.tsv")
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_pre() -> Result<(), Whatever> {
        test_fs_records::<FsPreService, FsPre>()
    }
}
