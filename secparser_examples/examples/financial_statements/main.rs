use secparser::downloader::DownloadConfigBuilder;
use secparser::financial_statements::financial_statements_files::FinancialStatementsFiles;

#[tokio::main]
async fn main() {
    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build();
    let files = FinancialStatementsFiles::new(download_config).await?;
}
