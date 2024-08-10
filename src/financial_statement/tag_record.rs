use anyhow::Result;
use serde::Deserialize;

use crate::deserializer::bool_from_int;

use super::data_source::FsDataSource;
use super::record::{FileIter, FsRecords, MaybeRecordIter};

#[derive(Debug, Deserialize)]
pub struct FsTag {
    pub tag: String,
    pub version: String,
    #[serde(deserialize_with = "bool_from_int")]
    pub custom: bool,
    #[serde(deserialize_with = "bool_from_int")]
    pub r#abstract: bool,
    pub datatype: String,
    pub iord: String,
    pub crdr: String,
    pub tlabel: String,
    pub doc: String,
}

pub struct FsTagRecords {
    file_iter: FileIter,
    maybe_record_iter: MaybeRecordIter<FsTag>,
}

impl FsRecords<FsTag> for FsTagRecords {
    fn get_file_iter_field(&mut self) -> &mut FileIter {
        &mut self.file_iter
    }

    fn get_maybe_record_iter_field(&mut self) -> &mut MaybeRecordIter<FsTag> {
        &mut self.maybe_record_iter
    }

    fn update_maybe_record_iter(&mut self, maybe_record_iter: MaybeRecordIter<FsTag>) {
        self.maybe_record_iter = maybe_record_iter;
    }
}

impl FsTagRecords {
    const TSV_FILENAME: &'static str = "tag.tsv";

    pub fn new(data_source: FsDataSource) -> Result<Self> {
        let mut file_iter = data_source.filepaths.into_iter();
        let maybe_record_iter = Self::get_maybe_record_iter(&mut file_iter, Self::TSV_FILENAME)?;

        Ok(Self {
            file_iter,
            maybe_record_iter,
        })
    }
}

impl Iterator for FsTagRecords {
    type Item = FsTag;

    fn next(&mut self) -> Option<Self::Item> {
        self.do_next(Self::TSV_FILENAME)
    }
}
