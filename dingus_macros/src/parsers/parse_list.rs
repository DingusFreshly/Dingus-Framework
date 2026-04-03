/*

input:


macro_call! {
    
    Position, 
}

 */

use syn::{Ident, parse::{Parse}};

pub struct ListParser {
    pub indents: Vec<Ident>,
}

impl Parse for ListParser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut indents = Vec::new();

        while !input.is_empty() {
            indents.push(input.parse()?);
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }

        Ok(ListParser { indents })
    }
}