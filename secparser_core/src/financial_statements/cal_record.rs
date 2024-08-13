use secparser_macros::FsRecordsImpl;
use serde::Deserialize;

const TSV_FILENAME: &str = "cal.tsv";

#[derive(Debug, Deserialize, FsRecordsImpl)]
pub struct FsCal {
    pub adsh: String,
    pub grp: Option<u8>,
    pub arc: Option<u8>,
    pub negative: Option<i8>,
    pub ptag: String,
    pub pversion: String,
    pub ctag: String,
    pub cversion: String,
}
