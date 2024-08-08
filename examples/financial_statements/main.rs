use anyhow::Result;
use secparser::downloader::DownloadConfigBuilder;
use secparser::financial_statement::data_source::FsDataSource;
use secparser::financial_statement::submission_record::FsSubmissionRecords;

fn main() -> Result<()> {
    env_logger::init();

    let user_agent = "example@secparser.com".to_string();
    let download_config = DownloadConfigBuilder::default()
        .user_agent(user_agent)
        .download_dir("./download".to_string())
        .build()?;
    let data_source = FsDataSource::get(download_config)?;

    let _ = FsSubmissionRecords::new(data_source)?;

    Ok(())
}
