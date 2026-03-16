use proc_macro_crate::{crate_name, FoundCrate};
use quote::format_ident;

pub fn dingus_path() -> proc_macro2::TokenStream {
    match crate_name("dingus_ecs") {
        Ok(FoundCrate::Itself) => quote::quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = format_ident!("{}", name);
            quote::quote!(#ident)
        }
        Err(_) => quote::quote!(dingus_ecs),
    }
}