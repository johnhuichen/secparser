use secparser_core::{
    cik_lookup::record::{CikLookup, CikLookupRecords},
    downloader::DownloadConfigBuilder,
};
use snafu::{ResultExt, Whatever};

use crate::{db::PostgresDB, progress_bar::CustomProgressBar};

pub fn ingest_cik_lookup() -> Result<(), Whatever> {
    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build()
        .whatever_context("Failed to create download config")?;

    let records =
        CikLookupRecords::new(&download_config).whatever_context("Failed to get records")?;
    let mut db = PostgresDB::new().whatever_context("Failed to get a db client")?;
    let bar = CustomProgressBar::new(records.count);

    for r in records {
        bar.inc_with_msg(1, &r.name);
        insert_cik_lookup(&mut db, r).whatever_context("Failed to insert record")?;
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
