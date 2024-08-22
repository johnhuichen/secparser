use secparser_core::{cik_lookup::record::CikLookupRecords, downloader::DownloadConfigBuilder};
use snafu::{ResultExt, Whatever};

fn main() -> Result<(), Whatever> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()
        .whatever_context("Failed to build download config")?;

    let records =
        CikLookupRecords::new(&download_config).whatever_context("Failed to get records")?;

    for r in records {
        log::info!("{r:?}");
    }

    Ok(())
}
