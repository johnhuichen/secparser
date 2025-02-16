use secparser_core::{
    cik_lookup::record::{CikLookup, CikLookupRecords},
    downloader::DownloadConfigBuilder,
};
use snafu::{ResultExt, Whatever};

use crate::ingestible::{IngestableRecordIter, IngestibleRecord, IngestibleRecordTable};

impl IngestibleRecord for CikLookup {
    fn display_name(&self) -> String {
        self.name.to_string()
    }
}

impl IngestableRecordIter for CikLookupRecords {
    type Item = CikLookup;
    type IntoIter = CikLookupRecords;

    fn get() -> Result<Self::IntoIter, Whatever> {
        let user_agent = "example@secparser.com".to_string();
        let download_config = DownloadConfigBuilder::default()
            .user_agent(user_agent)
            .build()
            .whatever_context("Failed to create download config")?;

        CikLookupRecords::new(&download_config).whatever_context("Failed to get records")
    }
}

pub struct CikLookupTable {}

impl IngestibleRecordTable for CikLookupTable {
    fn table_name() -> String {
        String::from("cik_lookup")
    }
}
