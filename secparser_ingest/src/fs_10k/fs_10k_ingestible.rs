use std::fs::File;

use csv::Writer;
use secparser_core::downloader::DownloadConfigBuilder;
use secparser_core::financial_statements::num_record::FsNum;
use secparser_core::financial_statements::record::{FsRecord, FsRecords};
use secparser_core::financial_statements::sub_record::FsSub;
use secparser_core::zip_csv_records::CsvConfigBuilder;
use snafu::ResultExt;

use crate::ingestible::{
    IngestableRecordIter as IngestibleRecordIter, IngestibleRecord, IngestibleRecordTable,
};

impl<T> IngestibleRecordIter for FsRecords<T>
where
    T: FsRecord + IngestibleRecord,
{
    type Item = T;
    type IntoIter = FsRecords<T>;

    fn get() -> Result<Self::IntoIter, snafu::Whatever> {
        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .build()
            .whatever_context("Failed to create download config")?;
        let csv_config = CsvConfigBuilder::default()
            .build()
            .whatever_context("Failed to build csv config")?;
        let from_year = 2009;

        let records: FsRecords<T> = FsRecords::new(&download_config, csv_config, from_year)
            .whatever_context("Failed to parse records")?;

        Ok(records)
    }
}

impl IngestibleRecord for FsSub {
    fn display_name(&self) -> String {
        self.name.to_string()
    }

    fn write_to_csv(&self, writer: &mut Writer<File>) -> Result<(), csv::Error> {
        if self.form != "10-K" {
            return Ok(());
        }

        writer.write_record(&[
            self.adsh.to_string(),
            self.cik.to_string(),
            self.ein.to_string(),
            self.afs.to_string(),
            self.fye.to_string(),
            self.form.to_string(),
            self.period.to_string(),
            self.fy.to_string(),
            self.fp.to_string(),
            self.filed.to_string(),
            self.instance.to_string(),
        ])?;

        Ok(())
    }
}

impl IngestibleRecord for FsNum {
    fn display_name(&self) -> String {
        format!("{}/{}/{}", self.adsh, self.tag, self.version)
    }

    fn write_to_csv(&self, writer: &mut Writer<File>) -> Result<(), csv::Error> {
        if self.iprx.unwrap_or_default() > 0 {
            return Ok(());
        }

        if self.dimh != "0x00000000" {
            return Ok(());
        }

        writer.write_record(&[
            self.adsh.to_string(),
            self.tag.to_string(),
            self.version.to_string(),
            self.ddate.to_string(),
            self.qtrs.unwrap_or_default().to_string(),
            self.uom.to_string(),
            self.value.unwrap_or_default().to_string(),
        ])?;

        Ok(())
    }
}

pub struct FsSubTable {}

impl IngestibleRecordTable for FsSubTable {
    fn table_name() -> String {
        String::from("fs_sub")
    }
}

pub struct FsNumTable {}

impl IngestibleRecordTable for FsNumTable {
    fn table_name() -> String {
        String::from("fs_num")
    }
    fn post_query() -> String {
        let post_query = "DELETE FROM tmp_table AS t
WHERE NOT EXISTS (
    SELECT * FROM fs_sub AS s
    WHERE s.adsh = t.adsh AND s.period = t.ddate
);";
        String::from(post_query)
    }
}
