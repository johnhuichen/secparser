use serde::{Deserialize, Serialize};

use super::record::FsRecord;

#[derive(Debug, Serialize, Deserialize)]
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

impl FsRecord for FsRen {
    fn csv_filename() -> String {
        "ren.tsv".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_ren() -> Result<(), Whatever> {
        test_fs_records::<FsRen>()
    }
}
