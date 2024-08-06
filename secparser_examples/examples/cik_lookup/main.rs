use secparser::cik_lookup::CikLookupRecords;

#[tokio::main]
async fn main() {
    let records = CikLookupRecords::new("example@secparser.com", "/tmp/secparser").await;

    for r in records {
        println!("{r:?}");
        break;
    }
}
