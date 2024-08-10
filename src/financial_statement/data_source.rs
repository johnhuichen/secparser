use anyhow::Result;
use std::path::PathBuf;

use chrono::{Datelike, Months, NaiveDate, Utc};

use crate::downloader::{DownloadConfig, Downloader};
use crate::traits::DataSource;

#[derive(Clone)]
pub struct FsDataSource {
    pub filepaths: Vec<PathBuf>,
}

impl FsDataSource {
    pub fn new(download_config: &DownloadConfig, from_date: NaiveDate) -> Result<Self> {
        let mut filepaths = Vec::new();

        let downloader = Downloader::new(download_config.clone());

        let urls = Self::get_urls(from_date);

        for url in urls {
            let zip_filepath = downloader.download(&url)?;

            filepaths.push(zip_filepath);
        }

        Ok(Self { filepaths })
    }

    fn get_urls(from_date: NaiveDate) -> Vec<String> {
        let mut result = Vec::new();

        let now = Utc::now();
        let year = now.year();
        let month = now.month();

        let one_year_ago = NaiveDate::from_ymd_opt(year - 1, month, 1).unwrap();

        let mut quarterly = from_date;
        while quarterly.checked_add_months(Months::new(3)).unwrap() < one_year_ago {
            let (_, year) = quarterly.year_ce();
            let quarter = match quarterly.month0() {
                0 => 1,
                3 => 2,
                6 => 3,
                9 => 4,
                _ => unreachable!(),
            };
            let url = format!(
                "https://www.sec.gov/files/dera/data/financial-statement-notes-data-sets/{}q{}_notes.zip",
                year,
                quarter);
            result.push(url);

            quarterly = quarterly.checked_add_months(Months::new(3)).unwrap();
        }

        let mut monthly = quarterly;

        let now = NaiveDate::from_ymd_opt(year, month, 1).unwrap();

        while monthly < now {
            let (_, year) = monthly.year_ce();
            let month = monthly.month0() + 1;
            let url = format!(
                "https://www.sec.gov/files/dera/data/financial-statement-notes-data-sets/{}_{:02}_notes.zip",
                year,
                month);
            result.push(url);

            monthly = monthly.checked_add_months(Months::new(1)).unwrap();
        }

        result
    }
}

impl DataSource for FsDataSource {
    fn validate_cache(&self) -> Result<()> {
        for filepath in &self.filepaths {
            Self::validate_non_empty_file(filepath)?;
        }

        Ok(())
    }
}
