use inquire::Select;
use snafu::{ResultExt, Whatever};

use self::cik_lookup::cik_lookup_menu;
use self::fs_10k::fs_10k_menu;

mod cik_lookup;
mod db;
mod fs_10k;
mod ingestible;
mod progress_bar;

#[snafu::report]
fn main() -> Result<(), Whatever> {
    env_logger::init();

    let cik_lookup_opt = "CIK lookup";
    let fs_10k_opt = "10-K";
    let exit_opt = "Exit";

    loop {
        let options: Vec<&str> = vec![cik_lookup_opt, fs_10k_opt, exit_opt];
        let ans = Select::new("Choose one of the options", options)
            .prompt()
            .unwrap_or_default();

        if ans == cik_lookup_opt {
            cik_lookup_menu::open().whatever_context("Error in cik lookup menu")?;
            continue;
        }
        if ans == fs_10k_opt {
            fs_10k_menu::open()
                .whatever_context("Error in financial ftatements submissions menu")?;
            continue;
        }
        if ans == exit_opt {
            break;
        }
    }

    Ok(())
}
