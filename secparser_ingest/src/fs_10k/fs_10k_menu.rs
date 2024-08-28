use colored::Colorize;
use inquire::Select;
use secparser_core::financial_statements::num_record::FsNum;
use secparser_core::financial_statements::record::FsRecords;
use secparser_core::financial_statements::sub_record::FsSub;
use secparser_core::financial_statements::tag_record::FsTag;
use snafu::{ResultExt, Whatever};

use crate::fs_10k::fs_10k_ingestible::{FsNumTable, FsSubTable, FsTagTable};
use crate::ingestible::{ingest, IngestableRecordIter};

pub fn open() -> Result<(), Whatever> {
    let mut maybe_records: Option<FsRecords<FsSub>> = None;

    let view_next_opt = "View Next Record";
    let ingest_opt = "Save to DB";
    let back_opt = "Back";

    loop {
        let options: Vec<&str> = vec![view_next_opt, ingest_opt, back_opt];
        let ans = Select::new("Choose one of the options", options)
            .prompt()
            .unwrap_or_default();

        if ans == back_opt {
            break;
        }

        if ans == view_next_opt {
            let records = match &mut maybe_records {
                Some(r) => r,
                None => {
                    let records = FsRecords::<FsSub>::get().whatever_context("test")?;
                    maybe_records.insert(records)
                }
            };

            loop {
                let maybe_record = records.next();

                match maybe_record {
                    Some(r) => {
                        if r.form == "10-K" {
                            println!("{}", format!("{r:#?}").green());
                            break;
                        }
                    }
                    None => {
                        println!("End of records");
                        break;
                    }
                }
            }

            continue;
        }

        if ans == ingest_opt {
            println!("Ingesting submission data");
            ingest::<FsRecords<FsSub>, FsSubTable>()
                .whatever_context("Failed to ingest submission data")?;

            println!("Ingesting numeric data");
            ingest::<FsRecords<FsNum>, FsNumTable>()
                .whatever_context("Failed to ingest numeric data")?;

            println!("Ingesting tag data");
            ingest::<FsRecords<FsTag>, FsTagTable>()
                .whatever_context("Failed to ingest numeric data")?;

            continue;
        }
    }

    Ok(())
}
