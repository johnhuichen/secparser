use std::error::Error;

mod downloader;
mod logger;
mod prompt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init()?;
    let url = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
    let result = downloader::download(url).await?;
    println!("{:#?}", result);

    Ok(())
}
