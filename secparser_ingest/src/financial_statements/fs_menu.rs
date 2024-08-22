use colored::Colorize;
use inquire::Select;
use secparser_core::financial_statements::record::FsRecords;
use snafu::{ResultExt, Whatever};

use crate::financial_statements::fs_ingest::ingest_fs_records;

use super::fs_ingest::{get_records, IngestableFsRecord};

pub fn open<T>() -> Result<(), Whatever>
where
    T: IngestableFsRecord,
{
    let mut maybe_records: Option<FsRecords<T>> = None;

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
                    let records = get_records().whatever_context("test")?;
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
            ingest_fs_records::<T>()
                .whatever_context("Failed to ingest financial statements data")?;

            continue;
        }

        unreachable!("Should not be here");
    }

    Ok(())
}
