use anyhow::Result;
use secparser::downloader::DownloadConfigBuilder;
use secparser::financial_statement::data_source::FsDataSource;
use secparser::financial_statement::tag_record::FsTagRecords;
use secparser::traits::DataSource;

fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()?;
    let data_source = FsDataSource::new(&download_config)?;
    data_source.validate_cache()?;
    log::info!("Data source cache is validated");

    let records = FsTagRecords::new(data_source)?;

    for record in records {
        log::info!("{:?}", record);
        break;
    }

    Ok(())
}
