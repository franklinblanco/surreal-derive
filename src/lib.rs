use derive::value::expand_derive_from_value;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derive;

#[proc_macro_derive(FromValue)]
pub fn from_value(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    match expand_derive_from_value(ident, data) {
        Ok(tokenstream) =>  tokenstream.into(),//panic!("{}", tokenstream.to_string()),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(ToValue)]
pub fn to_value(input: TokenStream) -> TokenStream {
    input
}