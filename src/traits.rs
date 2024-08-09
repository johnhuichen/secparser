use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

pub type FileLines = Lines<BufReader<File>>;

pub trait FileReader {
    fn get_lines(filepath: &PathBuf) -> Result<FileLines> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(file);

        Ok(reader.lines())
    }
}
