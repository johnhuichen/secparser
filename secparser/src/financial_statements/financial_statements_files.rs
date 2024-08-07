use core::panic;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::fs::create_dir_all;
use std::path::Path;

use csv::Writer;
use headless_chrome::Browser;

use crate::downloader::DownloadConfig;

pub struct FinancialStatementsFiles {}

impl FinancialStatementsFiles {
    pub async fn new(download_config: DownloadConfig) -> Self {
        FinancialStatementsFiles::get_urls(&download_config);

        FinancialStatementsFiles {}
    }

    fn get_urls(download_config: &DownloadConfig) -> HashSet<String> {
        create_dir_all(&download_config.download_dir)
            .unwrap_or_else(|e| panic!("Should create {}: {e}", download_config.download_dir));
        let filepath =
            Path::new(&download_config.download_dir).join("financial_statements_urls.csv");

        if download_config.use_local_cache && filepath.exists() {
            // TODO implement reader
            return HashSet::new();
        }

        let browser = Browser::default().unwrap_or_else(|e| panic!("Should get a browser: {e}"));
        let tab = browser
            .new_tab()
            .unwrap_or_else(|e| panic!("Should get a new tab: {e}"));

        let url = "https://www.sec.gov/data-research/financial-statement-notes-data-sets";
        tab.navigate_to(url)
            .unwrap_or_else(|e| panic!("Should navigate to {url}: {e}"));

        let selector = "a[href*='/files/dera/data/financial-statement-notes-data-sets/']";
        tab.wait_for_element(selector)
            .unwrap_or_else(|e| panic!("Should wait for element {selector}: {e}"));

        let elements = tab
            .find_elements(selector)
            .unwrap_or_else(|e| panic!("Should find elements {selector}: {e}"));
        let hrefs = elements
            .into_iter()
            .map(|e| e.attributes.expect("Should get element attributes"))
            .map(|e| e.get(1).expect("Should get href value").to_string())
            .map(|e| url.to_string() + &e)
            .collect::<HashSet<String>>();

        let mut writer = Writer::from_path(&filepath)
            .unwrap_or_else(|e| panic!("Should open {filepath:?}: {e}"));

        for href in hrefs.borrow() {
            writer
                .write_record([href])
                .unwrap_or_else(|e| panic!("Should write to {filepath:?}: {e}"));
        }
        writer
            .flush()
            .unwrap_or_else(|e| panic!("Should flush to {filepath:?}: {e}"));

        hrefs
    }
}
