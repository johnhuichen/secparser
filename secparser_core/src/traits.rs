use anyhow::Result;
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
