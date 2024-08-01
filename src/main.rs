use colored::Colorize;
use inquire::Select;

use self::local_config::LocalConfig;

mod csv_writer;
mod downloader;
mod local_config;
mod logger;
mod parse_cik;

#[tokio::main]
async fn main() {
    logger::init();

    let config_opt = "- View and edit local configuration";
    let cik_opt = "- Download and parse CIK lookup";
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
            .unwrap_or_else(|e| panic!("Should get a valid option: {e}"));

        if ans == config_opt {
            local_config.config_menu();
            continue;
        }

        if ans == cik_opt {
            parse_cik::parse().await;
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
            return;
        }
    }
}
