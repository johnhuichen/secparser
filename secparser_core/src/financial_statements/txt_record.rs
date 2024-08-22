use serde::Deserialize;

use crate::downloader::DownloadConfig;
use crate::zip_csv_records::CsvConfig;

use super::record::{FsRecords, FsRecordsError, FsService};

#[derive(Debug, Deserialize)]
pub struct FsTxt {
    pub adsh: String,
    pub tag: String,
    pub version: String,
    pub ddate: String,
    pub qtrs: Option<u16>,
    pub iprx: Option<u16>,
    pub lang: String,
    pub dcml: Option<u16>,
    pub durp: Option<f32>,
    pub datp: Option<f32>,
    pub dimh: String,
    pub dimn: Option<u8>,
    pub coreg: String,
    pub escaped: Option<u8>,
    pub srclen: Option<u32>,
    pub txtlen: Option<u32>,
    pub footnote: String,
    pub footlen: Option<u32>,
    pub context: String,
    pub value: String,
}

pub struct FsTxtService {}

impl FsService<FsTxt> for FsTxtService {
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<FsTxt>, FsRecordsError> {
        FsRecords::new(download_config, config, from_year, "txt.tsv")
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_txt() -> Result<(), Whatever> {
        test_fs_records::<FsTxtService, FsTxt>()
    }
}
