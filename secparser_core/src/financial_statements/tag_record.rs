use serde::Deserialize;

use crate::downloader::DownloadConfig;
use crate::zip_csv_records::CsvConfig;

use super::record::{FsRecords, FsRecordsError, FsService};

#[derive(Debug, Deserialize)]
pub struct FsTag {
    pub tag: String,
    pub version: String,
    pub custom: Option<u8>,
    pub r#abstract: Option<u8>,
    pub datatype: String,
    pub iord: String,
    pub crdr: String,
    pub tlabel: String,
    pub doc: String,
}

pub struct FsTagService {}

impl FsService<FsTag> for FsTagService {
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<FsTag>, FsRecordsError> {
        FsRecords::new(download_config, config, from_year, "tag.tsv")
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_tag() -> Result<(), Whatever> {
        test_fs_records::<FsTagService, FsTag>()
    }
}
