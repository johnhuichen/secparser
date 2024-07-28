use std::error::Error;

use self::downloader::Downloader;

mod downloader;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.sec.gov/Archives/edgar/cik-lookup-data.txt";
    let result = Downloader::download(url)?;
    println!("Hello, world!");

    Ok(())
}
