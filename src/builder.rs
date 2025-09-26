use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn implement(_: DeriveInput) -> TokenStream {
    quote! {}
}
