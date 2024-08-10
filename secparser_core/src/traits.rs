use anyhow::{Error, Result};
use encoding_rs::UTF_8;
use encoding_rs_io::{DecodeReaderBytes, DecodeReaderBytesBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::path::PathBuf;

pub type FileLines = Lines<BufReader<DecodeReaderBytes<File, Vec<u8>>>>;

pub trait FileReader {
    fn get_lines(filepath: &PathBuf) -> Result<FileLines> {
        let file = File::open(filepath)?;
        let reader = BufReader::new(
            DecodeReaderBytesBuilder::new()
                .encoding(Some(UTF_8))
                .build(file),
        );

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
