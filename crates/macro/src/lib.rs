use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Expr, ExprBlock, Token};

struct EventHandler {
    params: Vec<Expr>,
    body: ExprBlock,
}

impl Parse for EventHandler {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut params = vec![];
        loop {
            params.push(input.parse()?);
            if !input.peek(Token![,]) {
                break;
            } else {
                input.parse::<Token![,]>()?;
            }
        }
        input.parse::<Token![=>]>()?;
        let body = input.parse()?;
        Ok(EventHandler { params, body })
    }
}

#[proc_macro]
pub fn evt(input: TokenStream) -> TokenStream {
    let EventHandler { params, body } = parse_macro_input!(input as EventHandler);
    let params: Vec<&Expr> = params.iter().collect();
    let expanded = quote! {
        rtml::add!(#(#params),*)
            .evt(|evt, (#(#params),*)| #body)
    };
    TokenStream::from(expanded)
}
