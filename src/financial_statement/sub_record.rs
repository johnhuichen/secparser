use anyhow::Result;
use serde::Deserialize;

use super::data_source::FsDataSource;
use super::record::{FsRecords, FsRecordsConfig, FsRecordsIters, MaybeRecordIter};

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

pub struct FsSubRecords {
    iters: FsRecordsIters<FsSub>,
    config: FsRecordsConfig,
}

impl FsRecords<FsSub> for FsSubRecords {
    const TSV_FILENAME: &'static str = "sub.tsv";

    fn get_iters(&mut self) -> &mut FsRecordsIters<FsSub> {
        &mut self.iters
    }

    fn update_iters(&mut self, maybe_record_iter: MaybeRecordIter<FsSub>) {
        self.iters.maybe_record_iter = maybe_record_iter
    }

    fn get_config(&self) -> &FsRecordsConfig {
        &self.config
    }
}

impl FsSubRecords {
    pub fn new(data_source: FsDataSource, config: FsRecordsConfig) -> Result<Self> {
        let iters = Self::init_iters(data_source, &config)?;

        Ok(Self { iters, config })
    }
}

impl Iterator for FsSubRecords {
    type Item = FsSub;

    fn next(&mut self) -> Option<Self::Item> {
        self.do_next()
    }
}

#[cfg(test)]
mod tests {
    use crate::downloader::DownloadConfigBuilder;
    use crate::traits::DataSource;
    use anyhow::Result;

    use super::*;

    #[test]
    fn it_parses_fs_sub() -> Result<()> {
        env_logger::init();

        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .download_dir("./download".to_string())
            .build()?;

        let from_year = 2009;
        let data_source = FsDataSource::new(&download_config, from_year)?;
        data_source.validate_cache()?;
        log::info!("Data source cache is validated");

        let record_config = FsRecordsConfig { strict_mode: true };
        let records = FsSubRecords::new(data_source, record_config)?;

        for record in records {
            log::info!("{:?}", record);
        }

        Ok(())
    }
}
