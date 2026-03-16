use syn::{Ident, Result, parse::{Parse, ParseStream}};
use quote::format_ident;

pub struct ArchetypeParser {
    pub components: Vec<Vec<Ident>>,
    pub names: Vec<Ident>,
}

impl ArchetypeParser {

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn archetype_name(&self, i: usize) -> Ident {
        format_ident!("{}Archetype", self.names[i])
    }

    pub fn bundle_name(&self, i: usize) -> Ident {
        format_ident!("{}Bundle", self.names[i])
    }

    pub fn reg_name(&self, i: usize) -> Ident {
        format_ident!(
            "{}DESCRIPTOR_REG",
            self.names[i].to_string().to_uppercase()
        )
    }

    pub fn field_names(&self, i: usize) -> Vec<Ident> {
        self.components[i]
            .iter()
            .map(|c| format_ident!("{}", c.to_string().to_lowercase()))
            .collect()
    }

    pub fn component_types(&self, i: usize) -> &[Ident] {
        &self.components[i]
    }

    pub fn name(&self, i: usize) -> &Ident {
        &self.names[i]
    }
}

impl Parse for ArchetypeParser {

    fn parse(input: ParseStream) -> Result<Self> {
        let mut components = Vec::new();
        let mut names = Vec::new();

        while !input.is_empty() {

            let name: Ident = input.parse()?;
            names.push(name);

            let content;
            syn::bracketed!(content in input);

            let mut comp_list = Vec::new();

            while !content.is_empty() {
                comp_list.push(content.parse()?);

                if content.peek(syn::Token![,]) {
                    content.parse::<syn::Token![,]>()?;
                }
            }

            components.push(comp_list);

            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(ArchetypeParser { components, names })
    }
}