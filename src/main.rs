use colored::Colorize;
use inquire::Select;
use std::error::Error;

use self::local_config::LocalConfig;

mod downloader;
mod local_config;
mod logger;
mod parse_cik;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init()?;

    let config_opt = "- View and edit local configuration";
    let cik_opt = "- Download and parse CIK list";
    let financial_statement_opt = "- Download and parse Financial Statements";
    let form13f_opt = "- Download and parse Form 13F";
    let exit_opt = "- Exit";
    let mut local_config = LocalConfig::new();

    loop {
        let options = vec![
            config_opt,
            cik_opt,
            financial_statement_opt,
            form13f_opt,
            exit_opt,
        ];
        let ans = Select::new("Choose one of the options", options)
            .prompt()
            .unwrap_or_else(|_| panic!("Should get a valid option"));

        if ans == config_opt {
            local_config.config_menu();
            continue;
        }

        if ans == cik_opt {
            parse_cik::download_and_parse().await?;
            continue;
        }

        if ans == financial_statement_opt {
            log::info!("{}", "Not implemented!".bright_red());
            continue;
        }

        if ans == form13f_opt {
            log::info!("{}", "Not implemented!".bright_red());
            continue;
        }

        if ans == exit_opt {
            log::info!("Good bye");
            return Ok(());
        }
    }
}
