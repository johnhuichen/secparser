use secparser::cik_lookup::CikLookupRecords;
use secparser::downloader::DownloadConfigBuilder;

#[tokio::main]
async fn main() {
    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .build()
        .unwrap_or_else(|e| panic!("Should build download config: {e}"));
    let records = CikLookupRecords::get(download_config).await;

    for r in records {
        println!("{r:?}");
        break;
    }
}
