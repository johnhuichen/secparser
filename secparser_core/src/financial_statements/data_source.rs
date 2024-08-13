use chrono::{Datelike, Months, NaiveDate, Utc};
use snafu::Whatever;

use crate::data_source::DataSource;
use crate::downloader::DownloadConfig;

pub struct FsDataSources {
    pub vec: Vec<DataSource>,
}

impl FsDataSources {
    pub fn new(download_config: &DownloadConfig, from_year: i32) -> Result<Self, Whatever> {
        let data_sources = Self::get_urls(from_year)
            .into_iter()
            .map(|url| DataSource::new(download_config, &url).unwrap())
            .collect::<Vec<DataSource>>();

        Ok(FsDataSources { vec: data_sources })
    }

    fn get_urls(from_year: i32) -> Vec<String> {
        let mut result = Vec::new();

        let now = Utc::now();
        let year = now.year();
        let month = now.month();

        let one_year_ago = NaiveDate::from_ymd_opt(year - 1, month, 1).unwrap();

        let mut quarterly = NaiveDate::from_ymd_opt(from_year, 1, 1).unwrap();

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
