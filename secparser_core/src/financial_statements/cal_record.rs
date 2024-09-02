use serde::{Deserialize, Serialize};

use super::record::FsRecord;

#[derive(Debug, Serialize, Deserialize)]
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

impl FsRecord for FsCal {
    fn csv_filename() -> String {
        "cal.tsv".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_cal() -> Result<(), Whatever> {
        test_fs_records::<FsCal>()
    }
}
