use proc_macro2::Ident;
use quote::{format_ident, ToTokens};
use syn::Index;
use syn::parse::{Parse, ParseStream};

#[derive(Clone)]
pub enum FieldAccessor {
    Named(Ident),
    Unnamed(Index),
}
impl ToTokens for FieldAccessor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldAccessor::Named(ident) => ident.to_tokens(tokens),
            FieldAccessor::Unnamed(index) => index.to_tokens(tokens),
        }
    }
}
impl ToString for FieldAccessor {
    fn to_string(&self) -> String {
        match self {
            FieldAccessor::Named(ident) => ident.to_string(),
            FieldAccessor::Unnamed(index) => index.index.to_string(),
        }
    }
}

pub struct ComponentParse {
    pub ident : Ident,
    pub read_properties: Vec<FieldAccessor>,
    pub write_properties: Vec<FieldAccessor>,
}
impl Parse for ComponentParse {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse attributes first (they come before the struct keyword)
        let _attrs = input.call(syn::Attribute::parse_outer)?;

        let item = input.parse::<syn::ItemStruct>()?;
        let ident = item.ident;

        let mut read_properties = Vec::new();
        let mut write_properties = Vec::new();

        for (i, field) in item.fields.iter().enumerate() {
            let mut has_attr = false;
            let mut readonly = false;

            for attr in field.attrs.iter() {
                has_attr |= attr.path().is_ident("dingus_export");

                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("readonly") {
                        readonly = true;
                    }
                    Ok(())
                })?;
            }

            if has_attr {
                let mut field_ident;

                if let Some(ident) = &field.ident.clone() {
                    field_ident = FieldAccessor::Named(ident.clone());
                } else {
                    field_ident = FieldAccessor::Unnamed(Index::from(i));
                }

                read_properties.push(field_ident.clone());

                if !readonly {
                    write_properties.push(field_ident);
                }
            }
        }

        Ok(Self {
            ident,
            read_properties,
            write_properties,
        })
    }
}