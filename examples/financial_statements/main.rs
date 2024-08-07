use anyhow::Result;
use secparser::downloader::DownloadConfigBuilder;
use secparser::financial_statements::files::FinancialStatementsFiles;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build()?;
    let files = FinancialStatementsFiles::download(download_config).await?;
    // let urls = FinancialStatementsFiles::get();
    //
    // for url in urls {
    //     let submissions = FsSubmissions::get(download_config.clone(), &url).await;
    // }

    Ok(())
}
