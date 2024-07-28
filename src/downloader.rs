use std::fs;

pub struct Downloader {}

static TEMP_DIR: &str = "./tmp";

impl Downloader {
    pub fn download(url: &str) -> std::io::Result<String> {
        fs::create_dir_all(TEMP_DIR)?;

        let result = String::from("test");

        Ok(result)
    }
}
