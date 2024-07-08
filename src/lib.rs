use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// A simple derive macro to implement From<T> for JsValue.
#[proc_macro_derive(IntoJsValue)]
pub fn into_jsvalue_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_str = ident.to_string();
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Adding where clause constraints for all generic types to implement serde::Serialize
    let where_clause = match where_clause {
        Some(where_clause) => quote! { #where_clause },
        None => quote! {},
    };

    quote!(
        impl #impl_generics From<#ident #ty_generics> for wasm_bindgen::JsValue #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                let res = serde_wasm_bindgen::to_value(&value);
                wasm_bindgen::UnwrapThrowExt::expect_throw(
                    res,
                    &format!("failed to convert {} to a JsValue", #ident_str),
                )
            }
        }
    )
    .into()
}
