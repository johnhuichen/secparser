use anyhow::Result;
use csv::ReaderBuilder;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec;
use zip::ZipArchive;

pub type FileIter = vec::IntoIter<PathBuf>;
pub type MaybeRecordIter<T> = Option<vec::IntoIter<T>>;

pub trait FsRecords<I>
where
    I: DeserializeOwned,
{
    fn get_file_iter_field(&mut self) -> &mut FileIter;
    fn get_maybe_record_iter_field(&mut self) -> &mut MaybeRecordIter<I>;
    fn update_maybe_record_iter(&mut self, maybe_record_iter: MaybeRecordIter<I>);

    fn get_maybe_record_iter(
        file_iter: &mut FileIter,
        tsv_filename: &str,
    ) -> Result<MaybeRecordIter<I>> {
        match file_iter.next() {
            Some(filepath) => {
                let file = File::open(&filepath)?;
                let mut archive = ZipArchive::new(file)?;

                let tag_file = archive.by_name(tsv_filename)?;
                let reader = BufReader::new(tag_file);
                let reader = ReaderBuilder::new().delimiter(b'\t').from_reader(reader);
                let record_iter = reader
                    .into_deserialize()
                    .map(|r| r.unwrap_or_else(|e| panic!("Should parse tsv: {e}")))
                    .collect::<Vec<I>>()
                    .into_iter();

                Ok(Some(record_iter))
            }
            None => Ok(None),
        }
    }

    fn do_next(&mut self, tsv_filename: &str) -> Option<I> {
        loop {
            match self.get_maybe_record_iter_field() {
                Some(record_iter) => match record_iter.next() {
                    Some(v) => return Some(v),
                    None => {
                        let maybe_record_iter =
                            Self::get_maybe_record_iter(self.get_file_iter_field(), tsv_filename)
                                .unwrap_or_else(|e| panic!("Should get record iterator: {e}"));
                        self.update_maybe_record_iter(maybe_record_iter);
                    }
                },
                None => return None,
            }
        }
    }
}
