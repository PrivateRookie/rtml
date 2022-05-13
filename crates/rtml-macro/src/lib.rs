use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[derive(Debug, FromMeta)]
struct PageArgs {
    name: Option<String>,
}

#[proc_macro_attribute]
pub fn page(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let args = match PageArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let name = args.name.unwrap_or("index.html".to_string());
    let ident = input.sig.ident;
    let new_ident = syn::Ident::new(&format!("__wrapped_main_{}", ident.to_string()), ident.span());
    input.sig.ident = new_ident.clone();
    let expanded = quote! {
        #input

        fn __rtml_render_index<D: rtml::Markers + Clone>(page: rtml::tags::Html<D>) {
            use std::path::PathBuf;
            use std::fs::File;
            use std::io::Write;

            let page = page.to_string();
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let out_dir = std::env::var("RTML_OUT_DIR").unwrap_or_else(|_| "pkg".to_string());
            let file_path = path.join(out_dir);
            if !file_path.exists() {
                std::fs::create_dir_all(&file_path).unwrap();
            }
            let file_path = file_path.join(#name);
            let mut file = File::create(file_path).unwrap();
            file.write_all(page.as_bytes()).unwrap();
        }

        fn main() {
            __rtml_render_index(#new_ident());
        }
    };
    TokenStream::from(expanded)
}
