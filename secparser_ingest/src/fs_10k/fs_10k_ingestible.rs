use secparser_core::downloader::DownloadConfigBuilder;
use secparser_core::financial_statements::num_record::FsNum;
use secparser_core::financial_statements::record::{FsRecord, FsRecords};
use secparser_core::financial_statements::sub_record::FsSub;
use secparser_core::financial_statements::tag_record::FsTag;
use secparser_core::zip_csv_records::CsvConfigBuilder;
use snafu::ResultExt;

use crate::ingestible::{
    IngestableRecordIter as IngestibleRecordIter, IngestibleRecord, IngestibleRecordTable,
};

impl<T> IngestibleRecordIter for FsRecords<T>
where
    T: FsRecord + IngestibleRecord,
{
    type Item = T;
    type IntoIter = FsRecords<T>;

    fn get() -> Result<Self::IntoIter, snafu::Whatever> {
        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .build()
            .whatever_context("Failed to create download config")?;
        let csv_config = CsvConfigBuilder::default()
            .build()
            .whatever_context("Failed to build csv config")?;
        let from_year = 2009;

        let records: FsRecords<T> = FsRecords::new(&download_config, csv_config, from_year)
            .whatever_context("Failed to parse records")?;

        Ok(records)
    }
}

pub struct FsSubTable {}

impl IngestibleRecordTable for FsSubTable {
    fn table_name() -> String {
        String::from("fs_sub")
    }
}

impl IngestibleRecord for FsSub {
    fn display_name(&self) -> String {
        self.name.to_string()
    }
}

impl IngestibleRecord for FsNum {
    fn display_name(&self) -> String {
        format!("{}/{}/{}", self.adsh, self.tag, self.version)
    }
}

pub struct FsNumTable {}

impl IngestibleRecordTable for FsNumTable {
    fn table_name() -> String {
        String::from("fs_num")
    }
    fn post_query() -> String {
        let post_query = "DELETE FROM tmp_table AS t
WHERE NOT EXISTS (
    SELECT * FROM fs_sub AS s
    WHERE s.adsh = t.adsh AND s.period = t.ddate
);";
        String::from(post_query)
    }
}

pub struct FsTagTable {}

impl IngestibleRecordTable for FsTagTable {
    fn table_name() -> String {
        String::from("fs_tag")
    }
    fn post_query() -> String {
        let post_query = "DELETE FROM tmp_table AS t
WHERE NOT EXISTS (
    SELECT * FROM fs_num AS n
    WHERE n.tag = t.tag AND n.version = t.version
);";
        String::from(post_query)
    }
}

impl IngestibleRecord for FsTag {
    fn display_name(&self) -> String {
        format!("{}/{}", self.tag, self.version)
    }
}
