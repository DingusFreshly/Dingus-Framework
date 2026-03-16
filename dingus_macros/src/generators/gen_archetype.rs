use quote::quote;
use proc_macro2::TokenStream;
use crate::parsers::ArchetypeParser;
use crate::util::dingus_path;

pub fn generate_archetype_impl(parser: &ArchetypeParser, i: usize) -> TokenStream {

    let archetype_name = parser.archetype_name(i);
    let bundle_name = parser.bundle_name(i);
    let name = parser.name(i);
    let comp_types = parser.component_types(i);

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);

    quote! {
        pub struct #archetype_name;

        impl #dingus_internal::ArchetypeMarker for #archetype_name {

            const ARCHETYPE_ID: #dingus_internal::ArchetypeId =
                #i as #dingus_internal::ArchetypeId;

            const COMPONENT_SET: #dingus_internal::FastBit =
                #dingus_internal::FastBit::EMPTY
                #(.with(<#comp_types as #dingus_internal::ComponentTrait>::COMPONENT_INDEX))*;

            const NAME: &'static str = stringify!(#name);

            type Bundle = #bundle_name;
        }
    }
}


pub fn generate_bundle(parser: &ArchetypeParser, i: usize) -> TokenStream {

    let bundle_name = parser.bundle_name(i);
    let archetype_name = parser.archetype_name(i);

    let field_names = parser.field_names(i);
    let comp_types = parser.component_types(i);

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);

    quote! {

        pub struct #bundle_name {
            #(pub #field_names: #comp_types,)*
        }

        impl #dingus_internal::ArchetypeBundle for #bundle_name {

            type Marker = #archetype_name;

            unsafe fn write_into_archetype(
                self,
                archetype: &mut #dingus_internal::Archetype,
                row: usize
            ) {

                #(
                    let col = archetype.column_index[&<#comp_types as #dingus_internal::ComponentTrait>::component_type_id()];

                    (archetype.columns[col].ptr_at(row) as *mut #comp_types)
                        .write(self.#field_names);
                )*
            }
        }
    }
}


pub fn generate_linkme_block(parser: &ArchetypeParser, i: usize) -> TokenStream {

    let archetype_name = parser.archetype_name(i);
    let reg_name = parser.reg_name(i);
    let name = parser.name(i);

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);

    quote! {

        #[#dingus_internal::distributed_slice(#dingus_internal::ALL_ARCHETYPE_DESCRIPTORS)]
        static #reg_name: fn(&mut Vec<#dingus_internal::StaticArchetypeDescriptor>) =
            |v| v.push(#dingus_internal::StaticArchetypeDescriptor {

                archetype_id:     #i as #dingus_internal::ArchetypeId,
                name:             stringify!(#name),
                component_set:    <#archetype_name as #dingus_internal::ArchetypeMarker>::COMPONENT_SET,
                component_infos:  &[],
                initial_capacity: 0,

            });
    }
}


pub fn generate(parser: &ArchetypeParser) -> TokenStream {

    let mut bundles = Vec::new();
    let mut impls = Vec::new();
    let mut linkmes = Vec::new();

    for i in 0..parser.len() {

        bundles.push(generate_bundle(parser, i));
        impls.push(generate_archetype_impl(parser, i));
        linkmes.push(generate_linkme_block(parser, i));
    }

    quote! {
        #(#bundles)*
        #(#impls)*
        #(#linkmes)*
    }
}