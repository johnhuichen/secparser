use anyhow::Result;
use secparser_core::{
    downloader::DownloadConfigBuilder,
    financial_statement::{
        data_source::FsDataSource, record::FsRecordsConfigBuilder, tag_record::FsTagRecords,
    },
    traits::DataSource,
};

fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()?;
    let from_year = 2009;
    let data_source = FsDataSource::new(&download_config, from_year)?;
    data_source.validate_cache()?;
    log::info!("Data source cache is validated");

    // Create a config:
    // strict mode = true, program will panic if parse error occurs
    let record_config = FsRecordsConfigBuilder::default().build()?;

    // Get a list of tag records
    let records = FsTagRecords::new(data_source, record_config)?;

    for record in records {
        log::info!("{:?}", record);
        break;
    }

    Ok(())
}
