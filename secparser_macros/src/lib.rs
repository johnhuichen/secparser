use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FsRecordsImpl)]
pub fn derive_fs_records_impl(input: TokenStream) -> TokenStream {
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

fn records_impl_codegen(
    data_class: &proc_macro2::Ident,
    records_class: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    quote! {
        use anyhow::Result;
        use crate::financial_statement::data_source::FsDataSource;
        use crate::financial_statement::record::{FsRecords, FsRecordsConfig, FsRecordsIters, MaybeRecordIter};

        pub struct #records_class {
            iters: FsRecordsIters<#data_class>,
            config: FsRecordsConfig,
        }

        impl FsRecords<#data_class> for #records_class {
            const TSV_FILENAME: &'static str = TSV_FILENAME;

            fn get_iters(&mut self) -> &mut FsRecordsIters<#data_class> {
                &mut self.iters
            }

            fn update_iters(&mut self, maybe_record_iter: MaybeRecordIter<#data_class>) {
                self.iters.maybe_record_iter = maybe_record_iter
            }

            fn get_config(&self) -> &FsRecordsConfig {
                &self.config
            }
        }

        impl #records_class {
            pub fn new(data_source: FsDataSource, config: FsRecordsConfig) -> Result<Self> {
                let iters = Self::init_iters(data_source, &config)?;

                Ok(Self { iters, config })
            }
        }

        impl Iterator for #records_class {
            type Item = #data_class;

            fn next(&mut self) -> Option<Self::Item> {
                self.do_next()
            }
        }

    }
}

fn tests_codegen(
    data_class: &proc_macro2::Ident,
    records_class: &proc_macro2::Ident,
) -> proc_macro2::TokenStream {
    let data_class_name = format!("{}", data_class).to_lowercase()[2..].to_string();
    let test_fn_name = format!("it_parses_fs_{}", data_class_name);
    let test_fn = syn::Ident::new(&test_fn_name, data_class.span());

    quote! {
        #[cfg(test)]
        mod tests {
            use anyhow::Result;

            use crate::downloader::DownloadConfigBuilder;
            use crate::financial_statement::record::FsRecordsConfigBuilder;
            use crate::traits::DataSource;

            use super::*;

            #[test]
            fn #test_fn() -> Result<()> {
                env_logger::builder()
                    .is_test(true)
                    .try_init()
                    .unwrap_or_default();

                let user_agent = "example@secparser.com".to_string();
                let download_config = DownloadConfigBuilder::default()
                    .user_agent(user_agent)
                    .download_dir("./download".to_string())
                    .build()?;

                let from_year = 2024;
                let data_source = FsDataSource::new(&download_config, from_year)?;
                data_source.validate_cache()?;

                let record_config = FsRecordsConfigBuilder::default().eager_panic(true).build()?;
                let records = #records_class::new(data_source, record_config)?;

                for record in records {
                    log::debug!("{:?}", record);
                }

                Ok(())
            }
        }
    }
}
