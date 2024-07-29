use std::error::Error;

use crate::downloader;

pub async fn download_and_parse() -> Result<(), Box<dyn Error>> {
    let url = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
    let result = downloader::download(url).await?;
    println!("{:#?}", result);

    Ok(())
}
