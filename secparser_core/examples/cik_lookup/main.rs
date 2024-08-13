use anyhow::Result;
use secparser_core::{
    cik_lookup::{data_source::CikLookupDataSources, record::CikLookupRecords},
    downloader::DownloadConfigBuilder,
};

fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()?;

    let data_source = CikLookupDataSources::new(&download_config)?;
    let records = CikLookupRecords::new(data_source)?;

    for r in records {
        log::info!("{r:?}");
    }

    Ok(())
}
