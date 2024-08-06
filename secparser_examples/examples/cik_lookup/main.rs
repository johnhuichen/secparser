use secparser::cik_lookup::CikLookupRecords;

#[tokio::main]
async fn main() {
    let test = CikLookupRecords::new("example@secparser.com").await;

    for i in test {
        if !i.ticker.is_empty() {
            println!("{i:?}");
            panic!("I just want to see one record");
        }
    }
}
