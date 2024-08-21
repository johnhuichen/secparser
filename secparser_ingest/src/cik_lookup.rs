use indicatif::{ProgressBar, ProgressStyle};
use secparser_core::{
    cik_lookup::{
        data_source::CikLookupDataSources,
        record::{CikLookup, CikLookupRecords},
    },
    downloader::DownloadConfigBuilder,
};
use snafu::{ResultExt, Whatever};

use crate::db::PostgresDB;

pub fn ingest_cik_lookup() -> Result<(), Whatever> {
    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()
        .whatever_context("Failed to create download config")?;

    let data_source = CikLookupDataSources::new(&download_config)
        .whatever_context("Failed to create a data source")?;

    let records = CikLookupRecords::new(&data_source).whatever_context("Failed to get records")?;

    let bar = ProgressBar::new(records.count() as u64);
    bar.set_style(
        ProgressStyle::with_template("[{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    let records = CikLookupRecords::new(&data_source).whatever_context("Failed to get records")?;
    let mut db = PostgresDB::new().whatever_context("Failed to get a db client")?;

    for r in records {
        bar.set_message(r.name.to_string());
        insert_cik_lookup(&mut db, r).whatever_context("Failed to insert record")?;
        bar.inc(1);
    }
    bar.finish();

    Ok(())
}

fn insert_cik_lookup(db: &mut PostgresDB, record: CikLookup) -> Result<(), postgres::Error> {
    db.client.execute(
        "INSERT INTO cik_lookup AS t
(cik, name, ticker, exchange)
VALUES ($1, $2, $3, $4)
ON CONFLICT (cik) DO UPDATE 
SET   (ticker, exchange) =
      (COALESCE(t.ticker, EXCLUDED.ticker),
       COALESCE(t.exchange, EXCLUDED.exchange))
WHERE (EXCLUDED.ticker, EXCLUDED.exchange) IS DISTINCT FROM
      (COALESCE(t.ticker, EXCLUDED.ticker),
       COALESCE(t.exchange, EXCLUDED.exchange))
",
        &[
            &(record.cik as u32),
            &record.name,
            &record.ticker,
            &record.exchange,
        ],
    )?;

    Ok(())
}
