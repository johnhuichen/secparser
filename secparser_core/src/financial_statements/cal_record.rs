use serde::Deserialize;

use crate::downloader::DownloadConfig;
use crate::zip_csv_records::CsvConfig;

use super::record::{FsRecords, FsRecordsError, FsService};

#[derive(Debug, Deserialize)]
pub struct FsCal {
    pub adsh: String,
    pub grp: Option<u8>,
    pub arc: Option<u8>,
    pub negative: Option<i8>,
    pub ptag: String,
    pub pversion: String,
    pub ctag: String,
    pub cversion: String,
}

pub struct FsCalService {}

impl FsService<FsCal> for FsCalService {
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<FsCal>, FsRecordsError> {
        FsRecords::new(download_config, config, from_year, "cal.tsv")
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_cal() -> Result<(), Whatever> {
        test_fs_records::<FsCalService, FsCal>()
    }
}
