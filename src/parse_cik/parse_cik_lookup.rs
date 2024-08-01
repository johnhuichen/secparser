use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::{Path, PathBuf};

use crate::csv_writer::{write_csv, CsvRecords};
use crate::downloader::Downloader;
use crate::local_config::LocalConfig;

type FileLines = Lines<BufReader<File>>;

struct CikLookupIterator {
    iter: FileLines,
}

impl CikLookupIterator {
    fn new(lines: FileLines) -> Self {
        Self { iter: lines }
    }
}

impl Iterator for CikLookupIterator {
    type Item = [String; 2];

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(line) => {
                let line = line.unwrap_or_else(|e| panic!("Should get line in cik lookup: {e}"));
                let line = &line[..line.len() - 1];

                line.rsplit_once(":")
                    .map(|(company, cik)| [cik.to_string(), company.to_string()])
            }
            None => None,
        }
    }
}

struct CikLookupRecords {
    filepath: PathBuf,
}

impl CikLookupRecords {
    fn new(filepath: PathBuf) -> Self {
        Self { filepath }
    }

    fn get_lines(filepath: &PathBuf) -> FileLines {
        let file = File::open(filepath).unwrap_or_else(|e| panic!("Should open {filepath:?}: {e}"));
        let reader = BufReader::new(file);

        reader.lines()
    }
}

impl CsvRecords for CikLookupRecords {
    type LineIter = [String; 2];
    type RecordIter = CikLookupIterator;

    fn get_headers(&self) -> Vec<String> {
        vec!["cik".to_string(), "name".to_string()]
    }

    fn get_count(&self) -> u64 {
        let lines = Self::get_lines(&self.filepath);

        lines.count() as u64
    }

    fn get_iter(&self) -> Self::RecordIter {
        let lines = Self::get_lines(&self.filepath);

        CikLookupIterator::new(lines)
    }
}

pub async fn parse(downloader: &mut Downloader) {
    log::info!("Downloading cik lookup data");
    let local_config = LocalConfig::new();
    let url = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
    let filepath = downloader.download(url).await;

    log::info!("Parsing cik lookup data");
    let records = CikLookupRecords::new(filepath);
    let csv_path = Path::new(&local_config.out_dir).join("cik_lookup.csv");
    write_csv(&csv_path, records);
}
