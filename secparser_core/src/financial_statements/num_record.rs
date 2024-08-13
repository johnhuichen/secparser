use secparser_macros::FsRecordsImpl;
use serde::Deserialize;

const TSV_FILENAME: &str = "num.tsv";

#[derive(Debug, Deserialize, FsRecordsImpl)]
pub struct FsNum {
    pub adsh: String,
    pub tag: String,
    pub version: String,
    pub ddate: String,
    pub qtrs: Option<u16>,
    pub uom: String,
    pub dimh: String,
    pub iprx: Option<u16>,
    pub value: Option<f32>,
    pub footnote: String,
    pub footlen: Option<u32>,
    pub dimn: Option<u8>,
    pub coreg: String,
    pub durp: Option<f32>,
    pub datp: Option<f32>,
    pub dcml: Option<f32>,
}
