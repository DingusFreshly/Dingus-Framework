use quote::{format_ident, quote};
use syn::Ident;

use crate::util::dingus_path;

pub fn generate_component_impls(idents: Vec<Ident>) -> proc_macro2::TokenStream {

    let mut impls = Vec::new();

    let ecs = dingus_path();
    let dingus_internal = quote!(#ecs::internal);
    //impls.push(quote! { use #dingus_internal::ComponentTrait;});

    for (i, ident) in idents.iter().enumerate() {
        let linkme_name = format_ident!("_REGISTER_{}", ident);

        let impl_block = quote! {
            
            
            impl #dingus_internal::ComponentTrait for #ident {

                const COMPONENT_TYPE_ID: #dingus_internal::ComponentTypeId =
                    #i as #dingus_internal::ComponentTypeId;

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

        impls.push(impl_block);
    }

    quote! {
        #(#impls)*
    }
}