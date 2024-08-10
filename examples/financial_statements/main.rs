use anyhow::Result;
use chrono::NaiveDate;
use secparser::downloader::DownloadConfigBuilder;
use secparser::financial_statement::data_source::FsDataSource;
use secparser::financial_statement::record::FsRecordsConfig;
use secparser::financial_statement::tag_record::FsTagRecords;
use secparser::traits::DataSource;

fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()?;
    let from_date = NaiveDate::from_ymd_opt(2014, 1, 1).unwrap();
    let data_source = FsDataSource::new(&download_config, from_date)?;
    data_source.validate_cache()?;
    log::info!("Data source cache is validated");

    let record_config = FsRecordsConfig { strict_mode: true };
    let records = FsTagRecords::new(data_source, record_config)?;

    for record in records {
        log::info!("{:?}", record);
        break;
    }

    Ok(())
}
