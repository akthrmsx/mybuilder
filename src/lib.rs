use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod builder;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder::implement(input).into()
}
