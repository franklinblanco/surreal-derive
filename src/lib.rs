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

// surreal_query!(SQL string, TypeToReturn, Args/Obj)
// If it has a type to return then return that type in a result
// If it doesn't have a type then return a Vec<Response> 
// Destructure object into a Map and check that the query has named fields in case of an object query
// In case of arg query, then count arguments and put them in order inside the query.
// Start an in memory database and test that query on each build? Arg cancellable

/* 
#[proc_macro_derive(ToValue)]
pub fn to_value(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    match expand_derive_from_value(ident, data) {
        Ok(tokenstream) =>  tokenstream.into(),//panic!("{}", tokenstream.to_string()),
        Err(e) => e.into_compile_error().into(),
    }
    input
}*/