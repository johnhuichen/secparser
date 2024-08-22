use inquire::Select;
use snafu::{ResultExt, Whatever};

mod cik_lookup;
mod db;
mod financial_statements;
mod progress_bar;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    env_logger::init();

    let cik_lookup_opt = "Save CIK lookup to DB";
    let financial_statements_opt = "Save Financial Statements to DB";
    let exit_opt = "Exit";

    loop {
        let options: Vec<&str> = vec![cik_lookup_opt, financial_statements_opt, exit_opt];
        let ans = Select::new("Choose one of the options", options)
            .prompt()
            .whatever_context("Failed to get answer")?;

        if ans == cik_lookup_opt {
            cik_lookup::ingest_cik_lookup().whatever_context("Failed to ingest cik lookup")?;
        } else if ans == financial_statements_opt {
            financial_statements::sub::ingest_fs_sub()
                .whatever_context("Failed to ingest financial statements")?;
        } else if ans == exit_opt {
            break;
        }
    }

    Ok(())
}
