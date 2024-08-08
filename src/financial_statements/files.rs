use anyhow::Result;
use std::path::PathBuf;

use chrono::{Datelike, Months, NaiveDate, Utc};

use crate::downloader::{DownloadConfig, Downloader};

pub struct FsFiles {
    pub files: Vec<PathBuf>,
}

impl FsFiles {
    pub fn get_local_cache(download_config: DownloadConfig) -> Result<Self> {
        let downloader = Downloader::new(download_config);
        let urls = Self::get_urls();
        let files = urls
            .into_iter()
            .map(|u| downloader.get_filepath(&u).unwrap())
            .collect::<Vec<PathBuf>>();

        Ok(Self { files })
    }

    pub async fn download(download_config: DownloadConfig) -> Result<Self> {
        let mut files = Vec::new();

        let downloader = Downloader::new(download_config);

        let urls = Self::get_urls();
        for url in urls {
            let filepath = downloader.download(&url).await?;
            files.push(filepath);
        }

        Ok(Self { files })
    }

    fn get_urls() -> Vec<String> {
        let mut result = Vec::new();

        let now = Utc::now();
        let year = now.year();
        let month = now.month();

        let one_year_ago = NaiveDate::from_ymd_opt(year - 1, month, 1).unwrap();

        let mut quarterly = NaiveDate::from_ymd_opt(2009, 1, 1).unwrap();

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
