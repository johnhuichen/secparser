use serde::{Deserialize, Serialize};

use super::record::FsRecord;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsDim {
    pub dimhash: String,
    pub segments: String,
    pub segt: Option<u8>,
}

impl FsRecord for FsDim {
    fn csv_filename() -> String {
        "dim.tsv".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_dim() -> Result<(), Whatever> {
        test_fs_records::<FsDim>()
    }
}
