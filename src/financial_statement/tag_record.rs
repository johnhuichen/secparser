use anyhow::Result;
use serde::Deserialize;

use crate::deserializer::bool_from_int;

use super::data_source::FsDataSource;
use super::record::{FsRecords, FsRecordsConfig, FsRecordsIters, MaybeRecordIter};

#[derive(Debug, Deserialize)]
pub struct FsTag {
    pub tag: String,
    pub version: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub custom: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub r#abstract: bool,
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
    const TSV_FILENAME: &'static str = "tag.tsv";

    pub fn new(data_source: FsDataSource, config: FsRecordsConfig) -> Result<Self> {
        let mut file_iter = data_source.filepaths.into_iter();
        let maybe_record_iter =
            Self::get_maybe_record_iter(config.clone(), &mut file_iter, Self::TSV_FILENAME)?;
        let iters = FsRecordsIters {
            file_iter,
            maybe_record_iter,
        };

        Ok(Self { iters, config })
    }
}

impl Iterator for FsTagRecords {
    type Item = FsTag;

    fn next(&mut self) -> Option<Self::Item> {
        self.do_next(Self::TSV_FILENAME)
    }
}

#[cfg(test)]
mod tests {
    use crate::downloader::DownloadConfigBuilder;
    use crate::traits::DataSource;
    use anyhow::Result;
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn it_parses_fs_tag() -> Result<()> {
        env_logger::init();

        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .download_dir("./download".to_string())
            .build()?;

        let from_date = NaiveDate::from_ymd_opt(2009, 1, 1).unwrap();
        let data_source = FsDataSource::new(&download_config, from_date)?;
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
