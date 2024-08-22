use serde::Deserialize;

use crate::downloader::DownloadConfig;
use crate::zip_csv_records::CsvConfig;

use super::record::{FsRecords, FsRecordsError, FsService};

#[derive(Debug, Deserialize)]
pub struct FsDim {
    pub dimhash: String,
    pub segments: String,
    pub segt: Option<u8>,
}

pub struct FsDimService {}

impl FsService<FsDim> for FsDimService {
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<FsDim>, FsRecordsError> {
        FsRecords::new(download_config, config, from_year, "dim.tsv")
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_dim() -> Result<(), Whatever> {
        test_fs_records::<FsDimService, FsDim>()
    }
}
