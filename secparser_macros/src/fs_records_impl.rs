use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive_fs_records_impl_codegen(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Struct(_) => {
            let data_class = input.clone().ident;
            let records_class_name = format!("{}Records", data_class);
            let records_class = syn::Ident::new(&records_class_name, data_class.span());

            let records_impl_code = records_impl_codegen(&data_class, &records_class);
            let tests_code = tests_codegen(&data_class, &records_class);

            TokenStream::from(quote! {
                #records_impl_code
                #tests_code
            })
        }
        _ => TokenStream::from(
            syn::Error::new(
                input.ident.span(),
                "Only structs with named fields can derive `FsRecordsImpl`",
            )
            .to_compile_error(),
        ),
    }
}

pub fn records_impl_codegen(
    data_class: &proc_macro2::Ident,
    records_class: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        use std::vec;

        use crate::data_source::DataSource;
        use crate::financial_statements::data_source::FsDataSources;
        use crate::zip_csv_records::{CsvConfig, ZipCsvRecords, ZipCsvRecordsError};

        pub type DataSourceIter = vec::IntoIter<DataSource>;

        pub struct #records_class {
            pub config: CsvConfig,
            pub data_source_iter: DataSourceIter,
            pub maybe_records: Option<ZipCsvRecords<#data_class>>,
        }

        impl #records_class {
            pub fn new(data_sources: FsDataSources, config: CsvConfig) -> Result<Self, ZipCsvRecordsError> {
                let data_source_iter = data_sources.vec.into_iter();

                let mut result = Self {
                    config,
                    data_source_iter,
                    maybe_records: None,
                };

                result.get_maybe_record_iter()?;

                Ok(result)
            }

            fn get_maybe_record_iter(&mut self) -> Result<(), ZipCsvRecordsError> {
                match self.data_source_iter.next() {
                    Some(data_source) => {
                        let records: ZipCsvRecords<#data_class> =
                            ZipCsvRecords::new(&data_source, &self.config, TSV_FILENAME)?;

                        self.maybe_records = Some(records);

                        Ok(())
                    }
                    None => {
                        self.maybe_records = None;
                        Ok(())
                    },
                }
            }
        }

        impl Iterator for #records_class {
            type Item = #data_class;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match &mut self.maybe_records {
                        Some(record_iter) => match record_iter.next() {
                            Some(v) => return Some(v),
                            None => {
                                self.get_maybe_record_iter()
                                    .unwrap_or_else(|e| panic!("Should get record iterator: {e}"));
                            }
                        },
                        None => return None,
                    }
                }
            }
        }
    }
}

pub fn tests_codegen(
    data_class: &proc_macro2::Ident,
    records_class: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let data_class_name = format!("{}", data_class).to_lowercase()[2..].to_string();
    let test_fn_name = format!("it_parses_fs_{}", data_class_name);
    let test_fn = syn::Ident::new(&test_fn_name, data_class.span());

    quote! {
        #[cfg(test)]
        mod tests {
            use crate::{
                downloader::DownloadConfigBuilder,
                zip_csv_records::CsvConfigBuilder,
            };
            use snafu::{ResultExt, Whatever};

            use super::*;

            #[test]
            fn #test_fn() -> Result<(), Whatever> {
                env_logger::builder()
                    // .is_test(true)
                    .try_init()
                    .unwrap_or_default();

                let user_agent = "example@secparser.com".to_string();
                let download_config = DownloadConfigBuilder::default()
                    .user_agent(user_agent)
                    .download_dir("./download".to_string())
                    .build()
                    .whatever_context("Failed to build config")?;
                let from_year = 2024;
                let data_sources = FsDataSources::new(&download_config, from_year)?;

                let record_config = CsvConfigBuilder::default()
                    .panic_on_error(true)
                    .build()
                    .whatever_context("Failed to build csv config")?;
                let records = #records_class::new(data_sources, record_config)
                    .whatever_context("Failed to parse records")?;
                for record in records {
                    log::info!("{:?}", record);
                }

                Ok(())
            }
        }
    }
}
