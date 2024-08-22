use serde::Deserialize;

use crate::downloader::DownloadConfig;
use crate::zip_csv_records::CsvConfig;

use super::record::{FsRecords, FsRecordsError, FsService};

#[derive(Debug, Deserialize)]
pub struct FsNum {
    pub adsh: String,
    pub tag: String,
    pub version: String,
    pub ddate: String,
    pub qtrs: Option<u16>,
    pub uom: String,
    pub dimh: String,
    pub iprx: Option<u16>,
    pub value: Option<f32>,
    pub footnote: String,
    pub footlen: Option<u32>,
    pub dimn: Option<u8>,
    pub coreg: String,
    pub durp: Option<f32>,
    pub datp: Option<f32>,
    pub dcml: Option<f32>,
}

pub struct FsNumService {}

impl FsService<FsNum> for FsNumService {
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<FsNum>, FsRecordsError> {
        FsRecords::new(download_config, config, from_year, "num.tsv")
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_num() -> Result<(), Whatever> {
        test_fs_records::<FsNumService, FsNum>()
    }
}
