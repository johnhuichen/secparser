use serde::Deserialize;

use super::record::FsRecord;

#[derive(Debug, Deserialize)]
pub struct FsTag {
    pub tag: String,
    pub version: String,
    pub custom: Option<u8>,
    pub r#abstract: Option<u8>,
    pub datatype: String,
    pub iord: String,
    pub crdr: String,
    pub tlabel: String,
    pub doc: String,
}

impl FsRecord for FsTag {
    fn csv_filename() -> String {
        "tag.tsv".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::financial_statements::record::test_fs_records;
    use snafu::Whatever;

    use super::*;

    #[test]
    fn it_parses_fs_tag() -> Result<(), Whatever> {
        test_fs_records::<FsTag>()
    }
}
