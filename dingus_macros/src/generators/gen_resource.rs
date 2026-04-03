use syn::{Ident};
use quote::{quote};
use crate::util::dingus_path;
pub fn generate_resource_impls(idents: Vec<Ident>) -> proc_macro2::TokenStream {
    let dingus_path = dingus_path();
    let dingus_internal_path = quote! {#dingus_path::internal};
    let impls : Vec<_> = idents.iter().enumerate().map(
        |(i, ident)| {
            quote! {
                impl #dingus_internal_path::ResourceTrait for #ident {
                    const RESOURCE_INDEX: #dingus_internal_path::ResourceId = #i as #dingus_internal_path::ResourceId;
                    const NAME: &'static str = stringify!(#ident);
                }
            }
        }
    ).collect();
    quote! {
        #(#impls)*
    }
}