use secparser_core::downloader::DownloadConfigBuilder;
use secparser_core::financial_statements::record::FsService;
use secparser_core::financial_statements::sub_record::{FsSub, FsSubService};
use secparser_core::zip_csv_records::CsvConfigBuilder;
use snafu::{ResultExt, Whatever};

use crate::db::PostgresDB;
use crate::progress_bar::CustomProgressBar;

pub fn ingest_fs_sub() -> Result<(), Whatever> {
    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build()
        .whatever_context("Failed to create download config")?;
    let record_config = CsvConfigBuilder::default()
        .panic_on_error(true)
        .build()
        .whatever_context("Failed to build csv config")?;
    let from_year = 2009;

    let records = FsSubService::get_records(&download_config, record_config, from_year)
        .whatever_context("Failed to parse records")?;
    let mut db = PostgresDB::new().whatever_context("Failed to get a db client")?;
    let bar = CustomProgressBar::new(0);

    for r in records {
        bar.inc_with_msg(1, &r.name);
        insert_fs_sub(&mut db, r).whatever_context("Failed to insert record")?;
    }
    bar.finish();

    Ok(())
}

fn insert_fs_sub(db: &mut PostgresDB, record: FsSub) -> Result<(), postgres::Error> {
    db.client.execute(
        "INSERT INTO fs_sub AS t

(adsh, cik, name, sic,
countryba, stprba, cityba, zipba, bas1, bas2, baph,
countryma, stprma, cityma, zipma, mas1, mas2,
countryinc, stprinc,
ein, former, changed, afs, wksi, fye, form, period, fy, fp, filed, accepted,
prevrpt, detail, instance, nciks, aciks, pubfloatusd, floatdate, floataxis, floatmems)

VALUES ($1, $2, $3, $4,
       $5, $6, $7, $8, $9, $10, $11,
       $12, $13, $14, $15, $16, $17,
       $18, $19,
       $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31,
       $32, $33, $34, $35, $36, $37, $38, $39, $40)
ON CONFLICT ON CONSTRAINT fs_sub_pkey DO NOTHING",
        &[
            &record.adsh,
            &(record.cik as u32),
            &record.name,
            &record.sic,
            &record.countryba,
            &record.stprba,
            &record.cityba,
            &record.zipba,
            &record.bas1,
            &record.bas2,
            &record.baph,
            &record.countryma,
            &record.stprma,
            &record.cityma,
            &record.zipma,
            &record.mas1,
            &record.mas2,
            &record.countryinc,
            &record.stprinc,
            &record.ein,
            &record.former,
            &record.changed,
            &record.afs,
            &(record.wksi.unwrap_or_default() as i16),
            &record.fye,
            &record.form,
            &record.period,
            &record.fy,
            &record.fp,
            &record.filed,
            &record.accepted,
            &(record.prevrpt.unwrap_or_default() as i16),
            &(record.detail.unwrap_or_default() as i16),
            &record.instance,
            &(record.nciks.unwrap_or_default() as i16),
            &record.aciks,
            &record.pubfloatusd,
            &record.floatdate,
            &record.floataxis,
            &(record.floatmems.unwrap_or_default() as i16),
        ],
    )?;

    Ok(())
}
