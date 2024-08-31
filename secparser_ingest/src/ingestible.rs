use std::fs::{self, File};

use csv::Writer;
use itertools::Itertools;
use snafu::{whatever, ResultExt, Whatever};

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
    let query = "
    SELECT EXISTS (
    SELECT FROM information_schema.tables 
    WHERE  table_schema = 'public'
    AND    table_name   = $1
    );
    ";
    let row = db
        .client
        .query_one(query, &[&table_name])
        .whatever_context("Failed to check information schema")?;

    if !row.get::<&str, bool>("exists") {
        create_table(&mut db, &table_name)?;
    }

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

fn create_table(db: &mut PostgresDB, table_name: &str) -> Result<(), Whatever> {
    let query = if table_name == "cik_lookup" {
        "
        CREATE TABLE cik_lookup
        (
          cik OID NOT NULL,
          name TEXT,
          ticker TEXT,
          exchange TEXT,

          PRIMARY KEY (cik)
        );
        "
    } else if table_name == "fs_sub" {
        "
        CREATE TABLE fs_sub
        (
          adsh TEXT NOT NULL,
          cik OID,
          -- name TEXT,
          -- sic TEXT,

          /*
           *countryba TEXT,
           *stprba TEXT,
           *cityba TEXT,
           *zipba TEXT,
           *bas1 TEXT,
           *bas2 TEXT,
           *baph TEXT,
           *
           *
           * countryma TEXT,
           * stprma TEXT,
           * cityma TEXT,
           * zipma TEXT,
           * mas1 TEXT,
           * mas2 TEXT,
           *
           * countryinc TEXT,
           * stprinc TEXT,
           */

          ein TEXT,

          -- former TEXT, changed TEXT,

          afs TEXT,

          -- wksi SMALLINT,

          fye TEXT,
          form TEXT,
          period TEXT,
          fy TEXT,
          fp TEXT,
          filed TEXT,

          -- accepted TEXT,
          -- prevrpt SMALLINT NOT NULL,
          -- detail SMALLINT NOT NULL,
          instance TEXT,
          -- nciks SMALLINT NOT NULL,
          -- aciks TEXT,
          -- pubfloatusd REAL,
          -- floatdate TEXT,
          -- floataxis TEXT,
          -- floatmems SMALLINT,

          PRIMARY KEY (adsh)
        );
        "
    } else if table_name == "fs_num" {
        "
        CREATE TABLE fs_num
        (
          adsh TEXT NOT NULL,
          tag TEXT NOT NULL,
          version TEXT NOT NULL,
          ddate TEXT NOT NULL,
          qtrs SMALLINT NOT NULL,
          uom TEXT NOT NULL,
          -- dimh TEXT NOT NULL,
          -- iprx SMALLINT NOT NULL,
          value REAL,
          -- footnote TEXT,
          -- footlen BIGINT,
          -- dimn SMALLINT,
          -- coreg TEXT,
          -- durp REAL,
          -- datp REAL,
          -- dcml REAL,

          PRIMARY KEY (adsh, tag, version, ddate, qtrs, uom)
        );
        "
    } else if table_name == "fs_tag" {
        "
        CREATE TABLE fs_tag
        (
          tag TEXT NOT NULL,
          version TEXT NOT NULL,

          -- custom SMALLINT NOT NULL,

          abstract SMALLINT NOT NULL,
          datatype TEXT,
          iord TEXT,
          crdr TEXT,
          tlabel TEXT,

          -- doc TEXT,

          PRIMARY KEY (tag, version)
        );
        "
    } else {
        whatever!("Should create one of the tables: cik_lookup, fs_sub, fs_num, fs_tag");
    };

    db.client
        .batch_execute(query)
        .whatever_context(format!("Failed to create table {table_name}"))?;

    Ok(())
}
