use proc_macro::TokenStream;

use self::fs_records_impl::derive_fs_records_impl_codegen;

mod fs_records_impl;

#[proc_macro_derive(FsRecordsImpl)]
pub fn derive_fs_records_impl(input: TokenStream) -> TokenStream {
    derive_fs_records_impl_codegen(input)
}
