mod parsers;
mod generators;
mod util;

use proc_macro::TokenStream;
use parsers::{ListParser,ArchetypeParser, ArchetypeExportParse};
use quote::quote;
use syn::ItemStruct;

use crate::generators::{generate_component_impls, generate_resource_impls, generate_archetypes, derive_component, derive_archetype_export};
use crate::parsers::ComponentParse;

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

    let tokens = generate_archetypes(&input);

    TokenStream::from(
        quote! {
            #tokens
                
            //#expanded
        }
    )
}
#[proc_macro_derive(Component, attributes(dingus_export, readonly))]
pub fn component(item: TokenStream) -> TokenStream {

    let input = syn::parse_macro_input!(item as ComponentParse);

    let tokens = derive_component(input);

    TokenStream::from(
        quote! {
            #tokens
        }
    )
}
#[proc_macro_attribute]
pub fn archetype(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(attr as ArchetypeExportParse);
    let tokens = derive_archetype_export(&input);
    item.extend(
    TokenStream::from(
        quote! {
            
            #tokens
        }
    ));
    item    
}
