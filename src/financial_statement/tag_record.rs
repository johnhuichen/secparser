use csv::ReaderBuilder;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec;
use zip::ZipArchive;

use anyhow::Result;
use serde::Deserialize;

use crate::deserializer::bool_from_int;

use super::data_source::FsDataSource;

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

type FileIter = vec::IntoIter<PathBuf>;
type RecordIter = vec::IntoIter<FsTag>;

pub struct FsTagRecords {
    file_iter: FileIter,
    maybe_record_iter: Option<RecordIter>,
}

impl FsTagRecords {
    pub fn new(datasource: FsDataSource) -> Result<Self> {
        let mut file_iter = datasource.zip_files.into_iter();
        let maybe_record_iter = Self::get_record_iter(&mut file_iter)?;

        Ok(Self {
            file_iter,
            maybe_record_iter,
        })
    }

    pub fn get_record_iter(file_iter: &mut FileIter) -> Result<Option<RecordIter>> {
        match file_iter.next() {
            Some(filepath) => {
                let file = File::open(&filepath)
                    .unwrap_or_else(|e| panic!("Should open {filepath:?}: {e}"));
                let mut archive = ZipArchive::new(file)
                    .unwrap_or_else(|e| panic!("Should read zip file {filepath:?}: {e}"));

                let tag_file = archive
                    .by_name("tag.tsv")
                    .unwrap_or_else(|e| panic!("Should get tag.tsv: {e}"));
                let reader = BufReader::new(tag_file);
                let reader = ReaderBuilder::new().delimiter(b'\t').from_reader(reader);
                let record_iter = reader
                    .into_deserialize()
                    .map(|r| r.unwrap_or_else(|e| panic!("Should parse csv: {e}")))
                    .collect::<Vec<FsTag>>()
                    .into_iter();

                Ok(Some(record_iter))
            }
            None => Ok(None),
        }
    }
}

impl Iterator for FsTagRecords {
    type Item = FsTag;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.maybe_record_iter.as_mut() {
                Some(record_iter) => match record_iter.next() {
                    Some(v) => return Some(v),
                    None => {
                        let maybe_record_iter = Self::get_record_iter(&mut self.file_iter)
                            .unwrap_or_else(|e| panic!("Should get record iterator: {e}"));
                        self.maybe_record_iter = maybe_record_iter;
                    }
                },
                None => return None,
            }
        }
    }
}
