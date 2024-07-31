use std::path::PathBuf;

use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};

pub trait CsvRecords {
    type LineIter: IntoIterator<Item = String>;
    type RecordIter: IntoIterator<Item = Self::LineIter>;

    fn get_headers(&self) -> Vec<String>;
    fn get_count(&self) -> u64;
    fn get_iter(&self) -> Self::RecordIter;
}

pub fn write_csv(filepath: &PathBuf, records: impl CsvRecords) {
    let bar = ProgressBar::new(records.get_count());
    bar.set_style(
        ProgressStyle::with_template("[{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    let mut doc_writer = Writer::from_path(filepath)
        .unwrap_or_else(|e| panic!("Should open file {filepath:?}: {e}"));
    doc_writer
        .write_record(records.get_headers())
        .unwrap_or_else(|e| panic!("Should write to file {filepath:?}: {e}"));

    for record in records.get_iter() {
        doc_writer
            .write_record(record)
            .unwrap_or_else(|e| panic!("Should write to file {filepath:?}: {e}"));
        bar.inc(1);
    }

    bar.finish();
}
