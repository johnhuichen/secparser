use inquire::Select;
use secparser_core::financial_statements::dim_record::FsDim;
use secparser_core::financial_statements::num_record::FsNum;
use secparser_core::financial_statements::sub_record::FsSub;
use secparser_core::financial_statements::tag_record::FsTag;
use snafu::{ResultExt, Whatever};

use self::cik_lookup::cik_lookup_menu;
use self::financial_statements::fs_menu;

mod cik_lookup;
mod db;
mod financial_statements;
mod progress_bar;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    env_logger::init();

    let cik_lookup_opt = "CIK lookup";
    let fs_sub_opt = "Financial Statements Submissions";
    let fs_tag_opt = "Financial Statements Tag Data";
    let fs_dim_opt = "Financial Statements Dimensions Data";
    let fs_num_opt = "Financial Statements Numeric Data";
    let exit_opt = "Exit";

    loop {
        let options: Vec<&str> = vec![
            cik_lookup_opt,
            fs_sub_opt,
            fs_tag_opt,
            fs_dim_opt,
            fs_num_opt,
            exit_opt,
        ];
        let ans = Select::new("Choose one of the options", options)
            .prompt()
            .whatever_context("Failed to get answer")?;

        if ans == cik_lookup_opt {
            cik_lookup_menu::open().whatever_context("Error in cik lookup menu")?;
            continue;
        }
        if ans == fs_sub_opt {
            fs_menu::open::<FsSub>()
                .whatever_context("Error in financial ftatements submissions menu")?;
            continue;
        }
        if ans == fs_tag_opt {
            fs_menu::open::<FsTag>()
                .whatever_context("Error in financial ftatements tag data menu")?;
            continue;
        }
        if ans == fs_dim_opt {
            fs_menu::open::<FsDim>()
                .whatever_context("Error in financial ftatements dimension data menu")?;
            continue;
        }
        if ans == fs_num_opt {
            fs_menu::open::<FsNum>()
                .whatever_context("Error in financial ftatements numeric data menu")?;
            continue;
        }
        if ans == exit_opt {
            break;
        }
    }

    Ok(())
}
