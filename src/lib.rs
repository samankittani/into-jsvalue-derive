use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(IntoJsValue)]
pub fn into_jsvalue_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_str = ident.to_string();

    quote!(
    impl From<#ident> for wasm_bindgen::JsValue {
        fn from(value: #ident) -> Self {
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
