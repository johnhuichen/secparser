use anyhow::Result;
use secparser::cik_lookup::data_source::CikLookupDataSource;
use secparser::cik_lookup::record::CikLookupRecords;
use secparser::downloader::DownloadConfigBuilder;

fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()?;
    let datasource = CikLookupDataSource::get(&download_config)?;
    let records = CikLookupRecords::new(datasource)?;

    for r in records {
        log::info!("{r:?}");
        break;
    }

    Ok(())
}
