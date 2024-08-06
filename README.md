# SEC Parser

SEC Parser is a collection of scripts that parse datasets from https://sec.gov.

## Get Started

### Install Rust

SEC Parser is a rust application. You can follow the instructions on [official rust website](https://www.rust-lang.org/learn/get-started) to install rust.

### CIK Lookup

CIK lookup is the [current list of all CIKs matched with entity name](https://www.sec.gov/Archives/edgar/cik-lookup-data.txt). Note that this list includes funds and individuals and is historically cumulative for company names. Thus a given CIK may be associated with multiple names in the case of company or fund name changes, and the list contains some entities that no longer file with the SEC. [Reference](https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data)

In addition the parser also parses the [company_tickers_exchange.json](https://www.sec.gov/files/company_tickers_exchange.json) to get additonal ticker and exchange information for current tickers only.

See [source code](./secparser_examples/examples/cik_lookup/main.rs)

### Financial Statements

TBA

### Form 13F

TBA

## Feature Request

I will continue to include more parsing scripts. If you would like to have a dataset parsed, please submit a Git issue.

## Contribution

Contribution rules are TBA
