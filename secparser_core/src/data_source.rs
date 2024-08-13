use std::fs::File;
use std::io;
use std::path::PathBuf;

use snafu::{Location, ResultExt, Snafu};

use crate::downloader::{DownloadConfig, Downloader, DownloaderError};

#[derive(Debug, Snafu)]
pub enum DataSourceError {
    #[snafu(display("Cannot download"))]
    Downloader { source: DownloaderError },
}

#[derive(Debug, Snafu)]
pub enum ValidateError {
    #[snafu(display("IO error at {loc}"))]
    #[snafu(context(false))]
    IO {
        source: io::Error,
        #[snafu(implicit)]
        loc: Location,
    },

    #[snafu(display("File should not be empty: {filepath:?}"))]
    EmptyFile { filepath: PathBuf },
}

#[derive(Clone)]
pub struct DataSource {
    pub filepath: PathBuf,
}

impl DataSource {
    pub fn new(download_config: &DownloadConfig, url: &str) -> Result<Self, DataSourceError> {
        let downloader = Downloader::new(download_config.clone());
        let filepath = downloader.download(url).context(DownloaderSnafu)?;

        Ok(Self { filepath })
    }

    pub fn validate(&self) -> Result<(), ValidateError> {
        self.filepath.try_exists()?;

        let file = File::open(&self.filepath)?;
        let file_size = file.metadata()?.len();

        if file_size == 0 {
            EmptyFileSnafu {
                filepath: &self.filepath,
            }
            .fail()?;
        }

        Ok(())
    }
}
