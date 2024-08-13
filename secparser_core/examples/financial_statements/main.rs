use anyhow::Result;
use secparser_core::{
    downloader::DownloadConfigBuilder,
    financial_statement::{data_source::FsDataSources, tag_record::FsTagRecords},
    zip_csv_records::CsvConfigBuilder,
};

fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()?;
    let from_year = 2024;
    let data_sources = FsDataSources::new(&download_config, from_year)?;

    let record_config = CsvConfigBuilder::default().build()?;
    let records = FsTagRecords::new(data_sources, record_config)?;
    for record in records {
        log::info!("{:?}", record);
    }

    Ok(())
}
