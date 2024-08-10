use anyhow::Result;
use secparser_macros::FsRecordsImpl;
use serde::Deserialize;

use super::data_source::FsDataSource;
use super::record::{FsRecords, FsRecordsConfig, FsRecordsIters, MaybeRecordIter};

const TSV_FILENAME: &'static str = "tag.tsv";

#[derive(Debug, Deserialize, FsRecordsImpl)]
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
