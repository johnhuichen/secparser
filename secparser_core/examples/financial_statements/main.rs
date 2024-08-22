use secparser_core::financial_statements::record::FsRecords;
use secparser_core::financial_statements::sub_record::FsSub;
use secparser_core::{downloader::DownloadConfigBuilder, zip_csv_records::CsvConfigBuilder};
use snafu::{ResultExt, Whatever};

#[snafu::report]
fn main() -> Result<(), Whatever> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()
        .whatever_context("Failed to build download config")?;

    let csv_config = CsvConfigBuilder::default()
        .panic_on_error(true)
        .build()
        .whatever_context("Failed to build csv config")?;
    let from_year = 2024;
    let records: FsRecords<FsSub> = FsRecords::new(&download_config, csv_config, from_year)
        .whatever_context("Failed to parse records")?;
    for record in records {
        log::info!("{:?}", record);
    }

    Ok(())
}
