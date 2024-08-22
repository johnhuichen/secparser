use serde::Deserialize;

use crate::downloader::DownloadConfig;
use crate::zip_csv_records::CsvConfig;

use super::record::{FsRecords, FsRecordsError, FsService};

#[derive(Debug, Deserialize)]
pub struct FsRen {
    pub adsh: String,
    pub report: Option<u16>,
    pub rfile: String,
    pub menucat: String,
    pub shortname: String,
    pub longname: String,
    pub roleuri: String,
    pub parentroleuri: String,
    pub parentreport: Option<u16>,
    pub ultparentrpt: Option<u16>,
}

pub struct FsRenService {}

impl FsService<FsRen> for FsRenService {
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<FsRen>, FsRecordsError> {
        FsRecords::new(download_config, config, from_year, "ren.tsv")
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_ren() -> Result<(), Whatever> {
        test_fs_records::<FsRenService, FsRen>()
    }
}
