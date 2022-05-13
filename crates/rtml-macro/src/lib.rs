use darling::FromMeta;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, AttributeArgs, Ident, Item};

#[proc_macro]
pub fn check_attr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);
    let ret = if input.to_string().starts_with("on") {
        syn::Error::new(input.span(), "can not set event handler on attr! macro").to_compile_error()
    } else {
        input.to_token_stream()
    };
    ret.into()
}
