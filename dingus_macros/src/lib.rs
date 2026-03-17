mod parsers;
mod generators;
mod util;

use proc_macro2;
use proc_macro::TokenStream;
use parsers::{ListParser,ArchetypeParser};
use quote::quote;

use crate::generators::{generate_component_impls, generate_bundle, generate_linkme_block, generate_archetype_impl, generate_resource_impls, generate};

#[proc_macro]
pub fn include_components(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ListParser);
    let idents = input.indents;

    let expanded = generate_component_impls(idents);

    TokenStream::from(
        quote! {
            #expanded
        }
    )
}
#[proc_macro]
pub fn include_resources(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ListParser);
    let idents = input.indents;

    let expanded = generate_resource_impls(idents);

    TokenStream::from(
        quote! {
            #expanded
        }
    )
}

#[proc_macro]
pub fn include_archetypes(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ArchetypeParser);

    let tokens = generate(&input);

    TokenStream::from(
        quote! {
            #tokens
                
            //#expanded
        }
    )
}