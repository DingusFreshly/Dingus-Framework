use quote::{quote, ToTokens};
use proc_macro2::{Ident, TokenStream};
use syn::{ Expr, Path};
use crate::parsers::{ArchetypeExportParse, ArchetypeParser};
use crate::parsers::archetype_helpers::{archetype_name, bundle_name, field_names};
use crate::util::dingus_path;

pub fn generate_reg_loc_impl(name: &Ident, path: &Ident, i: usize ) -> TokenStream {
    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);
    quote! {

        impl #dingus_internal::RegisteredLocation for #path {
            const DENSE_INDEX: #dingus_internal::DenseIndex = #i as #dingus_internal::DenseIndex;
            const NAME : &'static str = stringify!(#name);
            const REGISTER_TYPE: #dingus_internal::RegisterType = #dingus_internal::RegisterType::Archetype;
        }
    }
}

pub fn generate_archetype_impl(parser: &ArchetypeParser, i: usize) -> TokenStream {

    let archetype_name = archetype_name(&parser.names[i]);
    let bundle_name = bundle_name(&parser.names[i]);
    let comp_types = &parser.components[i];

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);

    assert!(!comp_types.is_empty(), "archetype {:?} is empty", archetype_name);

    let archetype_def = quote! {
        pub struct #archetype_name;

        impl #dingus_internal::ArchetypeMarker for #archetype_name {

            const COMPONENT_SET: #dingus_internal::FastBit =
                #dingus_internal::FastBit::EMPTY
                #(.with(<#comp_types as #dingus_internal::ComponentTrait>::COMPONENT_TYPE_ID))*;

            type Bundle = #bundle_name;
        }
    };
    archetype_def
}


pub fn generate_bundle(bundle_name: &Ident, archetype_name: &Ident, field_names : &Vec<Ident>, comp_types : &Vec<Path>) -> TokenStream {

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);

    assert!(!comp_types.is_empty(), "archetype {:?} is empty", archetype_name);

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
                    let col = archetype.column_of(<#comp_types as #dingus_internal::ComponentTrait>::COMPONENT_TYPE_ID);

                    (archetype.columns[col].ptr_at(row) as *mut #comp_types)
                        .write(self.#field_names);
                )*
            }
        }
    }
}


pub fn generate_linkme_block(parser: &ArchetypeParser, i: usize) -> TokenStream {
    let reg_name = parser.reg_name(i);
    let component_types = &parser.components[i];
    let archetype_name = archetype_name(&parser.names[i]);
    let component_count = component_types.len();
    
    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);

    quote! {

        #[#dingus_internal::distributed_slice(#dingus_internal::ALL_ARCHETYPE_DESCRIPTORS)]
        static #reg_name: fn(&mut Vec<#dingus_internal::StaticArchetypeDescriptor>) =
            |v| {

                let archetype = <#archetype_name as #dingus_internal::ArchetypeMarker>::COMPONENT_SET;
                static COMPONENT_INFOS: std::sync::LazyLock<[#dingus_internal::ComponentInfo; #component_count]> = std::sync::LazyLock::new(||{ [
                    #(
                        <#component_types as #dingus_internal::ComponentTrait>::component_info()
                    ),*
                ]});
                v.push(#dingus_internal::StaticArchetypeDescriptor {
    
                    archetype_id:     #i as #dingus_internal::ArchetypeId,
                    name:             stringify!(#archetype_name),
                    component_set:    <#archetype_name as #dingus_internal::ArchetypeMarker>::COMPONENT_SET,
                    component_infos:  COMPONENT_INFOS.as_slice(),
                    initial_capacity: 0,
                    instance_def: <#archetype_name as #dingus_internal::ArchetypeMarker>::INSTANCE_DEF,
    
                }
            );
        };
    }
}



pub fn generate_archetypes(parser: &ArchetypeParser) -> TokenStream {
    let mut reg_locs = Vec::new();
    let mut bundles = Vec::new();
    let mut impls = Vec::new();
    let mut linkmes = Vec::new();

    for i in 0..parser.len() {

        let field_names = &field_names(&parser.components[i]);
        let comp_types = &parser.components[i];
        let archetype_name = &archetype_name(&parser.names[i]);
        let bundle_name = &bundle_name(&parser.names[i]);

        let is_empty = comp_types.is_empty();

        //assert!(!is_empty, "Archetype {:?} is empty", archetype_name );

        reg_locs.push(generate_reg_loc_impl(archetype_name, archetype_name, i));
        linkmes.push(generate_linkme_block(parser, i));

        if is_empty {
            bundles.push(TokenStream::new());
            impls.push(TokenStream::new());
        } else {
            bundles.push(generate_bundle(bundle_name, archetype_name, field_names, comp_types));
            impls.push(generate_archetype_impl(parser, i));
        }
    }

    quote! {

        #(#reg_locs)*
        #(#bundles)*
        #(#impls)*
        #(#linkmes)*
    }
}
pub fn generate_instance_def(parser: &ArchetypeExportParse) -> TokenStream {
    let name = &parser.name;

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);

    let property_names : Vec<Ident> = parser.properties.iter().map(|p| {p.key.clone()}).collect();
    let mut getters = vec![];
    let mut setters = vec![];
    let mut hints = vec![];

    for p in parser.properties.iter() {
        let value = &p.value;
        let path = &p.path;
        let hint = &p.ty_hint;
        hints.push(hint);

        let expr: Expr = syn::parse2(p.value.to_token_stream()).unwrap();

        let getter = quote! {
            |world: &#dingus_internal::World, entity: #dingus_internal::Entity| {
                let component = world.get::<#path>(entity).unwrap_or_else(|| panic!("Failed to get component for property {}. Is the entity missing the required component?", stringify!(#value)));
                Ok(#expr.into())
            }
        };
        let setter = if parser.readonly.contains(&p.key) {
            quote! {None}
        } else {
            quote! {
                Some(|world: &mut #dingus_internal::World, entity: #dingus_internal::Entity, value: #dingus_internal::DingusPrimitive| -> Result<(), #dingus_internal::ScriptError> {
                    let component = world.get_mut::<#path>(entity).unwrap_or_else(|| panic!("Failed to get component for property {}. Is the entity missing the required component?", stringify!(#value)));
                    let given_type = value.type_hint();
                    if let #dingus_internal::DingusTypeHint::#hint = given_type {
                        #expr = value.try_into()?;
                    } else {
                        return Err(#dingus_internal::ScriptError::UnexpectedType{
                            got: given_type,
                            expected: #dingus_internal::DingusTypeHint::#hint,
                        });
                    };
                    Ok(())
                })
            }
        };
        getters.push(getter);
        setters.push(setter);
    };

    let out = quote! {
        #dingus_internal::InstanceDef {
            class_name: stringify!(#name),
            properties: &[
                #(
                    #dingus_internal::PropertyDef {
                        name: stringify!(#property_names),
                        get: #getters,
                        set: #setters,
                        type_hint: #dingus_internal::DingusTypeHint::#hints,
                    }
                ),*
            ],
            methods: &[],
            base_classes: &[],
        }
    };
    out
}
pub fn derive_archetype_export(parser: &ArchetypeExportParse) -> TokenStream {
    let archetype_name = &parser.name;
    let instance_def = generate_instance_def(parser);
    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);
    let bundle_name = &bundle_name(archetype_name);
    let components = &parser.components;

    let bundle_tokens = generate_bundle(
        bundle_name,
        archetype_name,
        &field_names(&parser.components),
        components
    );

    let archetype_marker = quote! {
        impl #dingus_internal::ArchetypeMarker for #archetype_name {
            const COMPONENT_SET: #dingus_internal::FastBit = #dingus_internal::FastBit::EMPTY
                #(.with(<#components as #dingus_internal::ComponentTrait>::COMPONENT_TYPE_ID))*;

            type Bundle = #bundle_name;
            const INSTANCE_DEF: #dingus_internal::InstanceDef = #instance_def;
        }
    };

    quote! {
        #bundle_tokens
        #archetype_marker
    }
}