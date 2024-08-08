use anyhow::Result;
use secparser::cik_lookup::cik_lookup::CikLookupRecords;
use secparser::cik_lookup::files::CikLookupFiles;
use secparser::downloader::DownloadConfigBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build()?;
    let files = CikLookupFiles::download(download_config).await?;
    let records = CikLookupRecords::new(files)?;

    for r in records {
        println!("{r:?}");
        break;
    }

    Ok(())
}
