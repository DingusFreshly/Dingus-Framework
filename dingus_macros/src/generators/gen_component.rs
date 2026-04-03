use quote::{format_ident, quote};
use syn::{Ident, ItemStruct, LitStr, Path};
use crate::parsers::ComponentParse;
use crate::util::dingus_path;

pub fn generate_component_impls(idents: Vec<Ident>) -> proc_macro2::TokenStream {

    let mut impls = Vec::new();

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);
    //impls.push(quote! { use #dingus_internal::ComponentTrait;});

    for (i, ident) in idents.iter().enumerate() {
        let name = LitStr::new(ident.to_string().as_str(), ident.span());

        let impl_block = quote! {
            impl #dingus_internal::RegisteredLocation for #ident {

                const DENSE_INDEX: #dingus_internal::DenseIndex = #i as #dingus_internal::DenseIndex;
                const NAME: &'static str = #name;
                const REGISTER_TYPE : #dingus_internal::RegisterType = #dingus_internal::RegisterType::Component;
            }
        };

        impls.push(impl_block);
    }

    quote! {
        #(#impls)*
    }
}

pub fn derive_component(item: ComponentParse) -> proc_macro2::TokenStream {
    let ident = item.ident;
    let name = ident.to_string();

    let linkme_name = format_ident!("_REGISTER_{}", ident.to_string().to_uppercase());

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);

    let impl_block = quote! {

            impl #dingus_internal::ComponentTrait for #ident {
                
                fn component_info() -> #dingus_internal::ComponentInfo {
                    #dingus_internal::ComponentInfo {

                        type_id:  Self::COMPONENT_TYPE_ID,


                        layout:   std::alloc::Layout::new::<#ident>(),

                        drop_fn:  #dingus_internal::make_drop_fn::<#ident>(),

                        name:     std::any::type_name::<#ident>(),
                    }
                }

            }

            #[#dingus_internal::distributed_slice(#dingus_internal::ALL_COMPONENTS)]
            static #linkme_name: fn(&mut #dingus_internal::ComponentRegistry) =
                |reg| {
                    reg.register(
                        <#ident as #dingus_internal::ComponentTrait>::component_info()
                    );
            };

    };
    quote! {
        //#(#export_accessors)*

        #impl_block
    }
}

/*
struct based property retrieval
//let mut export_properties = Vec::new();
    //let mut export_accessors = Vec::new();

    for field in item.read_properties.iter() {
        let field_name = field.to_string();
        let getter_name = format_ident!("__dingus_get_{}", field_name);

        let getter = quote! {
            fn #getter_name(ptr: *mut u8) -> #dingus_internal::DingusPrimitive {
                let component_ptr = ptr as *mut #ident;
                let field_ref = unsafe { &(*component_ptr).#field };
                #dingus_internal::to_primitive(field_ref)
            }
        };
        //export_accessors.push(getter);

        let token = quote! {

            #dingus_internal::ExportPropertyInfo {
                name: #field_name,
                offset: std::mem::offset_of!(#ident, #field),
                get: #getter_name,
                set: None, // Read-only
            }
        };

        //export_properties.push(token);
    }
    for field in item.write_properties.iter() {
        let field_name = field.to_string();
        let getter_name = format_ident!("__dingus_get_{}", field_name);

        let getter = quote! {
            unsafe fn #getter_name(ptr: *mut u8) -> Box<#dingus_internal::DingusPrimitive> {
                let component_ptr = ptr as *mut #ident;
                let field_ref = unsafe { &(*component_ptr).#field };
                Box::new(#dingus_internal::to_primitive(field_ref))
            }
        };
        let setter_name = format_ident!("__dingus_set_{}", field_name);
        let setter = quote! {
            unsafe fn #setter_name(ptr: *mut u8, value: #dingus_internal::DingusPrimitive) {
                let component_ptr = ptr as *mut #ident;
                let field_ref = unsafe { &mut (*component_ptr).#field };
                *field_ref = #dingus_internal::from_primitive(value);
            }
        };

        //export_accessors.push(setter);
        //export_accessors.push(getter);

        let token = quote! {
            #dingus_internal::ExportPropertyInfo {
                name: #field_name,
                offset: std::mem::offset_of!(#ident, #field),
                get: #getter_name,
                set: Some(#setter_name),
            }
        };

        //export_properties.push(token);
    }
 */