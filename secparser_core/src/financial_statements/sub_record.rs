use serde::Deserialize;

use crate::downloader::DownloadConfig;
use crate::zip_csv_records::CsvConfig;

use super::record::{FsRecords, FsRecordsError, FsService};

#[derive(Debug, Deserialize)]
pub struct FsSub {
    pub adsh: String,
    pub cik: usize,
    pub name: String,
    pub sic: String,

    pub countryba: String,
    pub stprba: String,
    pub cityba: String,
    pub zipba: String,
    pub bas1: String,
    pub bas2: String,
    pub baph: String,

    pub countryma: String,
    pub stprma: String,
    pub cityma: String,
    pub zipma: String,
    pub mas1: String,
    pub mas2: String,

    pub countryinc: String,
    pub stprinc: String,

    pub ein: String,
    pub former: String,
    pub changed: String,
    pub afs: String,
    pub wksi: Option<u8>,
    pub fye: String,
    pub form: String,
    pub period: String,
    pub fy: String,
    pub fp: String,
    pub filed: String,
    pub accepted: String,
    pub prevrpt: Option<u8>,
    pub detail: Option<u8>,
    pub instance: String,
    pub nciks: Option<u16>,
    pub aciks: String,
    pub pubfloatusd: Option<f32>,
    pub floatdate: String,
    pub floataxis: String,
    pub floatmems: Option<u8>,
}

pub struct FsSubService {}

impl FsService<FsSub> for FsSubService {
    fn get_records(
        download_config: &DownloadConfig,
        config: CsvConfig,
        from_year: i32,
    ) -> Result<FsRecords<FsSub>, FsRecordsError> {
        FsRecords::new(download_config, config, from_year, "sub.tsv")
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_sub() -> Result<(), Whatever> {
        test_fs_records::<FsSubService, FsSub>()
    }
}
