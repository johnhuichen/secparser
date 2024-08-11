use secparser_macros::FsRecordsImpl;
use serde::Deserialize;

const TSV_FILENAME: &str = "dim.tsv";

#[derive(Debug, Deserialize, FsRecordsImpl)]
pub struct FsDim {
    pub dimhash: String,
    pub segments: String,
    pub segt: Option<u8>,
}
