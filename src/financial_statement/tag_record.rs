use anyhow::Result;
use serde::Deserialize;

use super::data_source::FsDataSource;
use super::record::{FsRecords, FsRecordsConfig, FsRecordsIters, MaybeRecordIter};

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

pub struct FsTagRecords {
    iters: FsRecordsIters<FsTag>,
    config: FsRecordsConfig,
}

impl FsRecords<FsTag> for FsTagRecords {
    const TSV_FILENAME: &'static str = "tag.tsv";

    fn get_iters(&mut self) -> &mut FsRecordsIters<FsTag> {
        &mut self.iters
    }

    fn update_iters(&mut self, maybe_record_iter: MaybeRecordIter<FsTag>) {
        self.iters.maybe_record_iter = maybe_record_iter
    }

    fn get_config(&self) -> &FsRecordsConfig {
        &self.config
    }
}

impl FsTagRecords {
    pub fn new(data_source: FsDataSource, config: FsRecordsConfig) -> Result<Self> {
        let iters = Self::init_iters(data_source, &config)?;

        Ok(Self { iters, config })
    }
}

impl Iterator for FsTagRecords {
    type Item = FsTag;

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
    fn it_parses_fs_tag() -> Result<()> {
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
        let records = FsTagRecords::new(data_source, record_config)?;

        for record in records {
            log::info!("{:?}", record);
        }

        Ok(())
    }
}
