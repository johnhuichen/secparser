use colored::Colorize;
use inquire::Select;
use secparser_core::cik_lookup::record::CikLookupRecords;
use secparser_core::downloader::DownloadConfigBuilder;
use snafu::{ResultExt, Whatever};

use crate::cik_lookup::cik_lookup_ingest::ingest_cik_lookup;

pub fn open() -> Result<(), Whatever> {
    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build()
        .whatever_context("Failed to create download config")?;

    let mut maybe_records: Option<CikLookupRecords> = None;

    let view_next_opt = "View Next Record";
    let ingest_opt = "Save to DB";
    let back_opt = "Back";

    loop {
        let options: Vec<&str> = vec![view_next_opt, ingest_opt, back_opt];
        let ans = Select::new("Choose one of the options", options)
            .prompt()
            .whatever_context("Failed to get answer")?;

        if ans == back_opt {
            break;
        }

        if ans == view_next_opt {
            let records = match &mut maybe_records {
                Some(r) => r,
                None => {
                    let records = CikLookupRecords::new(&download_config)
                        .whatever_context("Failed to get records")?;
                    maybe_records.insert(records)
                }
            };

            let maybe_record = records.next();
            match maybe_record {
                Some(r) => println!("{}", format!("{r:#?}").green()),
                None => println!("End of records"),
            }

            continue;
        }

        if ans == ingest_opt {
            ingest_cik_lookup().whatever_context("Failed to ingest CIK lookup")?;

            continue;
        }

        unreachable!("Should not be here");
    }

    Ok(())
}
