use anyhow::Result;
use csv::ReaderBuilder;
use derive_builder::Builder;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec;
use zip::ZipArchive;

use super::data_source::FsDataSource;

pub type FileIter = vec::IntoIter<PathBuf>;
pub type MaybeRecordIter<T> = Option<vec::IntoIter<T>>;

pub struct FsRecordsIters<T> {
    pub maybe_record_iter: MaybeRecordIter<T>,
    pub file_iter: FileIter,
}

#[derive(Clone, Debug, Builder)]
pub struct FsRecordsConfig {
    #[builder(default = "true")]
    pub csv_flexible: bool,
    #[builder(default = "false")]
    pub csv_quoting: bool,
    #[builder(default = "false")]
    pub eager_panic: bool,
}

pub trait FsRecords<T>
where
    T: DeserializeOwned,
{
    const TSV_FILENAME: &'static str;

    fn get_iters(&mut self) -> &mut FsRecordsIters<T>;
    fn update_iters(&mut self, maybe_record_iter: MaybeRecordIter<T>);
    fn get_config(&self) -> &FsRecordsConfig;

    fn get_tsv_filename() -> &'static str {
        Self::TSV_FILENAME
    }

    fn init_iters(
        data_source: FsDataSource,
        config: &FsRecordsConfig,
    ) -> Result<FsRecordsIters<T>> {
        let mut file_iter = data_source.filepaths.into_iter();
        let maybe_record_iter = Self::get_maybe_record_iter(config.clone(), &mut file_iter)?;

        Ok(FsRecordsIters {
            file_iter,
            maybe_record_iter,
        })
    }

    fn get_maybe_record_iter(
        config: FsRecordsConfig,
        file_iter: &mut FileIter,
    ) -> Result<MaybeRecordIter<T>> {
        match file_iter.next() {
            Some(filepath) => {
                let file = File::open(&filepath)?;
                let mut archive = ZipArchive::new(file)?;

                let tag_file = archive.by_name(Self::get_tsv_filename())?;
                let reader = BufReader::new(tag_file);
                let reader = ReaderBuilder::new()
                    .quoting(config.csv_quoting)
                    .flexible(config.csv_flexible)
                    .delimiter(b'\t')
                    .from_reader(reader);
                let record_iter = reader
                    .into_deserialize()
                    .map(|r| {
                        r.map_err(|e| {
                            if config.eager_panic {
                                panic!("Should parse {filepath:?}: {e}");
                            } else {
                                log::error!("Should parse {filepath:?}: {e}");
                            }
                        })
                    })
                    .filter_map(|r| r.ok())
                    .collect::<Vec<T>>()
                    .into_iter();

                Ok(Some(record_iter))
            }
            None => Ok(None),
        }
    }

    fn do_next(&mut self) -> Option<T> {
        loop {
            match self.get_iters().maybe_record_iter.as_mut() {
                Some(record_iter) => match record_iter.next() {
                    Some(v) => return Some(v),
                    None => {
                        let maybe_record_iter = Self::get_maybe_record_iter(
                            self.get_config().clone(),
                            &mut self.get_iters().file_iter,
                        )
                        .unwrap_or_else(|e| panic!("Should get record iterator: {e}"));
                        self.update_iters(maybe_record_iter);
                    }
                },
                None => return None,
            }
        }
    }
}
