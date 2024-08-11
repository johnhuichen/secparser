use secparser_macros::FsRecordsImpl;
use serde::Deserialize;

const TSV_FILENAME: &str = "txt.tsv";

#[derive(Debug, Deserialize, FsRecordsImpl)]
pub struct FsTxt {
    pub adsh: String,
    pub tag: String,
    pub version: String,
    pub ddate: String,
    pub qtrs: Option<u16>,
    pub iprx: Option<u16>,
    pub lang: String,
    pub dcml: Option<u16>,
    pub durp: Option<f32>,
    pub datp: Option<f32>,
    pub dimh: String,
    pub dimn: Option<u8>,
    pub coreg: String,
    pub escaped: Option<u8>,
    pub srclen: Option<u32>,
    pub txtlen: Option<u32>,
    pub footnote: String,
    pub footlen: Option<u32>,
    pub context: String,
    pub value: String,
}
