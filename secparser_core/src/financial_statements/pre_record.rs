use secparser_macros::FsRecordsImpl;
use serde::Deserialize;

const TSV_FILENAME: &str = "pre.tsv";

#[derive(Debug, Deserialize, FsRecordsImpl)]
pub struct FsPre {
    pub adsh: String,
    pub report: Option<u16>,
    pub line: Option<u32>,
    pub stmt: String,
    pub inpth: Option<u8>,
    pub tag: String,
    pub version: String,
    pub prole: String,
    pub plabel: String,
    pub negating: Option<u8>,
}
