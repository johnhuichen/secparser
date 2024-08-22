use std::fmt::Debug;

use secparser_core::{
    downloader::DownloadConfigBuilder,
    financial_statements::record::{FsRecord, FsRecords},
    zip_csv_records::CsvConfigBuilder,
};
use serde::de::DeserializeOwned;
use snafu::{ResultExt, Whatever};

use crate::db::PostgresDB;
use crate::progress_bar::CustomProgressBar;

pub fn ingest_fs_records<T>(
    insert_record: fn(db: &mut PostgresDB, record: &T) -> Result<(), postgres::Error>,
    get_display_name: fn(record: &T) -> String,
) -> Result<(), Whatever>
where
    T: Debug + FsRecord + DeserializeOwned,
{
    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build()
        .whatever_context("Failed to create download config")?;
    let csv_config = CsvConfigBuilder::default()
        .build()
        .whatever_context("Failed to build csv config")?;
    let from_year = 2020;

    let records: FsRecords<T> = FsRecords::new(&download_config, csv_config, from_year)
        .whatever_context("Failed to parse records")?;
    let mut db = PostgresDB::new().whatever_context("Failed to get a db client")?;
    let bar = CustomProgressBar::new(0);

    for r in records {
        bar.inc_with_msg(1, &get_display_name(&r));
        if let Err(e) = insert_record(&mut db, &r) {
            log::error!("Failed to insert {r:?}: {e}");
        }
    }
    bar.finish();

    Ok(())
}
