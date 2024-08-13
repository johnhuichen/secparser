use secparser_core::{
    downloader::DownloadConfigBuilder,
    financial_statements::{data_source::FsDataSources, tag_record::FsTagRecords},
    zip_csv_records::CsvConfigBuilder,
};
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
    let from_year = 2024;
    let data_sources = FsDataSources::new(&download_config, from_year)?;

    let record_config = CsvConfigBuilder::default()
        .error_on_parse_failure(true)
        .build()
        .whatever_context("Failed to build csv config")?;
    let records = FsTagRecords::new(data_sources, record_config)
        .whatever_context("Failed to parse records")?;
    for record in records {
        log::info!("{:?}", record);
    }

    Ok(())
}
