use serde::{Deserialize, Serialize};

use super::record::FsRecord;

#[derive(Debug, Serialize, Deserialize)]
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

impl FsRecord for FsPre {
    fn csv_filename() -> String {
        "pre.tsv".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_pre() -> Result<(), Whatever> {
        test_fs_records::<FsPre>()
    }
}
