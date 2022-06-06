use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn macro_builder(input: TokenStream) -> TokenStream {
    let name = parse_macro_input!(input as Ident);
    let expanded = quote! {
        #[macro_export]
        macro_rules! #name {
            (
                $($(#$($a_cap:ident),+)? $(#)?{ $($($a_name:ident)-+ $(= $a_value:expr)?)*  })?
                $($(*$($s_cap:ident),+)? $(*)?{ $($($s_name:ident)-+: $s_value:expr);* $(;)? })?
                $(:$($b_cap:ident),+ => $b_body:expr;)?
                $(@$type:ident = $($e_cap:ident),+ => $b:expr;)*
                $(|$($v_cap:ident),+|  $content:expr)?
                ) => {
                    {
                        let __tag__ = rtml::tags::#name(());
            
                        $(
                            let __tag__ = __tag__.attr(rtml::t_attr! { $($($a_cap),+   #> )? $($($a_name)-+ $(=$a_value)?),*});
                        )?
                        $(
                            let __tag__ = __tag__.style(rtml::style_! { $($($s_cap),+ *>)? $($($s_name)-+ :$s_value);*});
                        )?
                        $(
                            let __tag__ = __tag__.bind(rtml::ref_subs!($($b_cap),+ :> $b_body));
                        )?
                        $(
                            let __tag__ = __tag__.children(rtml::ref_subs!($($v_cap),+ => $content));
                        )?
                        $(
                            let __tag__ = __tag__.on(stringify!($type), rtml::ref_update!($($e_cap),+ => $b));
                        )*
                        __tag__
                    }
                };
                (
                $($(#$($a_cap:ident),+)? $(#)?{ $($($a_name:ident)-+ $(= $a_value:expr)?)*  })?
                $($(*$($s_cap:ident),+)? $(*)?{ $($($s_name:ident)-+: $s_value:expr);* $(;)? })?
                $(:$($b_cap:ident),+ => $b_body:expr;)?
                $(@$type:ident = $($e_cap:ident),+ => $b:expr;)*
                $(|| $content:expr)?
                ) => {
                    {
                        let __tag__ = rtml::tags::#name(());
            
                        $(
                            let __tag__ = __tag__.attr(rtml::t_attr! { $($($a_cap),+   #> )? $($($a_name)-+ $(=$a_value)?),*});
                        )?
                        $(
                            let __tag__ = __tag__.style(rtml::style_! { $($($s_cap),+ *>)? $($($s_name)-+ :$s_value);*});
                        )?
                        $(
                            let __tag__ = __tag__.bind(rtml::ref_subs!($($b_cap),+ :> $b_body));
                        )?
                        $(
                            let __tag__ = __tag__.children($content);
                        )?
                        $(
                            let __tag__ = __tag__.on(stringify!($type), rtml::ref_update!($($e_cap),+ => $b));
                        )*
                        __tag__
                    }
                };
        }
    };
    TokenStream::from(expanded)
}
