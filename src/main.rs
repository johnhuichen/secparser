use colored::Colorize;
use inquire::Select;
use std::error::Error;

mod downloader;
mod logger;
mod parse_cik;

#[derive(Debug)]
enum MenuOptions {
    CIK,
    FinancialStatement,
    Form13f,
    Exit,
}

impl MenuOptions {
    const CIK_DESC: &'static str = "Download and parse CIK list";
    const FINANCIAL_STATEMENT_DESC: &'static str = "Download and parse Financial Statements";
    const FORM13F_DESC: &'static str = "Download and parse Form 13F";

    const EXIT_DESC: &'static str = "Exit";

    fn from(s: &str) -> Self {
        match s {
            Self::CIK_DESC => Self::CIK,
            Self::FINANCIAL_STATEMENT_DESC => Self::FinancialStatement,
            Self::FORM13F_DESC => Self::Form13f,
            Self::EXIT_DESC => Self::Exit,
            _ => panic!("Should be one of menu options"),
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::CIK => Self::CIK_DESC,
            Self::FinancialStatement => Self::FINANCIAL_STATEMENT_DESC,
            Self::Form13f => Self::FORM13F_DESC,
            Self::Exit => Self::EXIT_DESC,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init()?;

    let cik = MenuOptions::CIK.as_str();
    let financial_statement = MenuOptions::FinancialStatement.as_str();
    let form13f = MenuOptions::Form13f.as_str();
    let exit = MenuOptions::Exit.as_str();

    let options: Vec<&str> = vec![cik, financial_statement, form13f, exit];

    loop {
        let ans = Select::new("Choose one of the options", options.clone())
            .prompt()
            .unwrap_or_else(|_| panic!("Should get a valid option"));

        match MenuOptions::from(ans) {
            MenuOptions::CIK => parse_cik::download_and_parse().await?,
            MenuOptions::FinancialStatement => println!("{}", "Not implemented!".bright_red()),
            MenuOptions::Form13f => println!("{}", "Not implemented!".bright_red()),
            MenuOptions::Exit => return Ok(()),
        }
    }
}
