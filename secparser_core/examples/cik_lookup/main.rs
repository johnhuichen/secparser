use anyhow::Result;
use secparser_core::{
    cik_lookup::{data_source::CikLookupDataSource, record::CikLookupRecords},
    downloader::DownloadConfigBuilder,
    traits::DataSource,
};

fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()?;

    let data_source = CikLookupDataSource::new(&download_config)?;
    data_source.validate_cache()?;
    log::info!("Data source cache is validated");

    let records = CikLookupRecords::new(data_source)?;

    for r in records {
        log::info!("{r:?}");
        break;
    }

    Ok(())
}
