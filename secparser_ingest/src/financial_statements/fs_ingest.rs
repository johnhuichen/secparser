use secparser_core::{
    downloader::DownloadConfigBuilder,
    financial_statements::record::{FsRecord, FsRecords},
    zip_csv_records::CsvConfigBuilder,
};
use snafu::{ResultExt, Whatever};

use crate::{db::PostgresDB, progress_bar::CustomProgressBar};

pub trait IngestableFsRecord: FsRecord {
    fn display_name(&self) -> String;
    fn insert(&self, db: &mut PostgresDB) -> Result<(), postgres::Error>;
}

pub fn get_records<T>() -> Result<FsRecords<T>, Whatever>
where
    T: IngestableFsRecord,
{
    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build()
        .whatever_context("Failed to create download config")?;
    let csv_config = CsvConfigBuilder::default()
        .build()
        .whatever_context("Failed to build csv config")?;
    let from_year = 2024;

    let records: FsRecords<T> = FsRecords::new(&download_config, csv_config, from_year)
        .whatever_context("Failed to parse records")?;

    Ok(records)
}

pub fn ingest_fs_records<T>() -> Result<(), Whatever>
where
    T: IngestableFsRecord,
{
    let records: FsRecords<T> = get_records()?;
    let mut db = PostgresDB::new().whatever_context("Failed to get a db client")?;
    let bar = CustomProgressBar::new(0);

    for r in records {
        bar.inc_with_msg(1, &r.display_name());
        if let Err(e) = r.insert(&mut db) {
            log::error!("Failed to insert {r:?}: {e}");
        }
    }
    bar.finish();

    Ok(())
}
