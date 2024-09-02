use serde::{Deserialize, Serialize};

use super::record::FsRecord;

#[derive(Debug, Serialize, Deserialize)]
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

impl FsRecord for FsTxt {
    fn csv_filename() -> String {
        "txt.tsv".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_txt() -> Result<(), Whatever> {
        test_fs_records::<FsTxt>()
    }
}
