use secparser_macros::FsRecordsImpl;
use serde::Deserialize;

const TSV_FILENAME: &str = "ren.tsv";

#[derive(Debug, Deserialize, FsRecordsImpl)]
pub struct FsRen {
    pub adsh: String,
    pub report: Option<u16>,
    pub rfile: String,
    pub menucat: String,
    pub shortname: String,
    pub longname: String,
    pub roleuri: String,
    pub parentroleuri: String,
    pub parentreport: Option<u16>,
    pub ultparentrpt: Option<u16>,
}
