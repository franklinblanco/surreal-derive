
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, Field, FieldsNamed, FieldsUnnamed, Ident};

#[allow(dead_code, unused)]
pub fn expand_derive_from_value(ident: Ident, data: Data) -> syn::Result<TokenStream> {
    let fields: Vec<Field> = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named.into_iter().collect(),
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => return Err(syn::Error::new(
                ident.span(),
                "Tuple structs are not yet supported.",
            )),
            syn::Fields::Unit => {
                return Err(syn::Error::new(
                    ident.span(),
                    "Unit structs are not yet supported.",
                ))
            }
        },
        _ => return Err(syn::Error::new(ident.span(), "Only structs are allowed.")),
    };
    let constructor: Vec<TokenStream> = fields.into_iter().map(|field| destructure_field(field.ident.unwrap())).collect();
    
    let output = quote! {
    impl TryFrom<surrealdb::sql::Value> for #ident {
        type Error = surrealdb::Error;

        fn try_from(value: surrealdb::sql::Value) -> Result<Self, Self::Error> {
            match value {
                surrealdb::sql::Value::Object(object) => Ok(Self { #(#constructor), * }),
                surrealdb::sql::Value::Array(ref array) => {
                    if array.len() > 0 {
                        return Self::try_from(array.get(0).unwrap().clone());
                    } else {
                        return Err(surrealdb::Error::QueryEmpty);
                    }
                },
                _ => Err(surrealdb::Error::TryFromError(value.to_string(), "Value not an object")),
            }
        }
    }
    };

    Ok(output.into())
}

/*#[allow(dead_code, unused)]
pub fn expand_derive_into_value(input: TokenStream) -> syn::Result<TokenStream> {
    let fields: Vec<Field> = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(FieldsNamed { named, .. }) => named.into_iter().collect(),
            syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => return Err(syn::Error::new(
                ident.span(),
                "Tuple structs are not yet supported.",
            )),
            syn::Fields::Unit => {
                return Err(syn::Error::new(
                    ident.span(),
                    "Unit structs are not yet supported.",
                ))
            }
        },
        _ => return Err(syn::Error::new(ident.span(), "Only structs are allowed.")),
    };
    let constructor: Vec<TokenStream> = fields.into_iter().map(|field| destructure_field(field.ident.unwrap())).collect();
    
    let output = quote! {
    impl From<#ident> for surrealdb::sql::Value{
        type Error = surrealdb::Error;

        fn try_from(value: surrealdb::sql::Value) -> Result<Self, Self::Error> {
            match value {
                surrealdb::sql::Value::Object(object) => Ok(Self { #(#constructor), * }),
                surrealdb::sql::Value::Array(ref array) => {
                    if array.len() > 0 {
                        return Self::try_from(array.get(0).unwrap().clone());
                    } else {
                        return Err(surrealdb::Error::TryFromError(value.to_string(), "Value contains an empty array"));
                    }
                },
                _ => Err(surrealdb::Error::TryFromError(value.to_string(), "Value not an object")),
            }
        }
    }
    };

    Ok(output.into())
} */

fn destructure_field(field_ident: Ident) -> TokenStream {
    let str_field_name = field_ident.to_string();
    // In case it's id, then the path is Object>thing>id>id type (can be anything)
    /*if str_field_name.eq("id".into()) {
        return quote!{
            #field_ident: match object.0.get(#str_field_name) {
                Some(val) => 
                    match val.clone() {
                        surrealdb::sql::Value::Thing(thing) => 
                            thing.id.into(),
                        _ => {
                            return Err(())
                        }
                    },
                None => {
                    return Err(())
                }  
            }
        }
    }*/
    quote!{
        #field_ident: 
            match object.0.get(#str_field_name) {
                Some(val) => match val.clone() {
                    surrealdb::sql::Value::Thing(thing) => thing.id.to_raw(),
                    _ => val.clone().try_into()?
                }, 
                None => return Err(surrealdb::Error::TryFromError(object.to_string(), "Key not found in BtreeMap")) 
            }
        }
}