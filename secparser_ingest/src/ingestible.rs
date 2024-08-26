use std::fs::{self, File};

use csv::Writer;
use itertools::Itertools;
use snafu::{ResultExt, Whatever};

use crate::db::PostgresDB;
use crate::progress_bar::CustomProgressBar;

pub trait IngestibleRecordTable {
    fn table_name() -> String;
    fn post_query() -> String {
        String::from("")
    }
}

pub trait IngestibleRecord {
    fn display_name(&self) -> String;
    fn write_to_csv(&self, writer: &mut Writer<File>) -> Result<(), csv::Error>;
}

pub trait IngestableRecordIter {
    type Item: IngestibleRecord;
    type IntoIter: Iterator<Item = Self::Item>;

    fn get() -> Result<Self::IntoIter, Whatever>;
}

pub fn ingest<I, T>() -> Result<(), Whatever>
where
    I: IngestableRecordIter,
    T: IngestibleRecordTable,
{
    let csv_dir = "/tmp/";
    let chunk_size = 1_000_000;
    let table_name = T::table_name();
    let post_query = T::post_query();

    let mut db = PostgresDB::new().whatever_context("Failed to get a db client")?;
    let records = I::get()?;
    let csv_path = format!("{}{}.csv", csv_dir, table_name);

    let bar = CustomProgressBar::new(0);
    for chunk in &records.chunks(chunk_size) {
        let mut writer =
            Writer::from_path(&csv_path).whatever_context("Failed to get csv writer")?;
        for record in chunk {
            bar.inc_with_msg(1, &record.display_name());
            record
                .write_to_csv(&mut writer)
                .whatever_context("Failed to write to csv")?;
        }

        writer.flush().whatever_context("Failed to flush csv")?;
        copy_from_csv(&mut db, &csv_path, &table_name, &post_query)
            .whatever_context("Failed to copy from csv")?;
    }
    bar.finish();

    fs::remove_file(&csv_path).whatever_context("Failed to delete csv file")?;

    Ok(())
}

pub fn copy_from_csv(
    db: &mut PostgresDB,
    csv_path: &str,
    table_name: &str,
    post_query: &str,
) -> Result<(), postgres::Error> {
    let query = format!(
        "BEGIN;
CREATE TEMP TABLE tmp_table
(LIKE {1} INCLUDING DEFAULTS)
ON COMMIT DROP;

COPY tmp_table FROM '{0}' DELIMITER ',' QUOTE '\"' NULL '' CSV;

{2}

INSERT INTO {1}
SELECT *
FROM tmp_table
ON CONFLICT DO NOTHING;
COMMIT;
",
        csv_path, table_name, post_query
    );
    db.client.batch_execute(&query)
}
