use anyhow::{Error, Result};
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

pub trait DataSource {
    fn validate_cache(&self) -> Result<()>;

    fn validate_non_empty_file(filepath: &PathBuf) -> Result<()> {
        if !filepath.exists() {
            return Err(Error::msg(format!("Should have {filepath:?}")));
        }

        let file = File::open(filepath)?;
        let file_size = file.metadata()?.len();

        if file_size == 0 {
            return Err(Error::msg(format!(
                "Should have non empty file {filepath:?}"
            )));
        }

        Ok(())
    }
}
