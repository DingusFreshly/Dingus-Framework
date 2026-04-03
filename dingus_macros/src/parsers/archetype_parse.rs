use syn::{Ident, Result, parse::{Parse, ParseStream}, LitStr, Expr, bracketed, Token, Path};
use quote::{format_ident, ToTokens};
use syn::token::Token;

fn ident_to_lit_str(ident: &Ident) -> LitStr {
    LitStr::new(&ident.to_string(), ident.span())
}

#[derive(Clone)]
pub struct Property {
    pub key: Ident,
    pub path:Path,
    pub value: Expr,
    pub ty_hint : Ident,
}

impl Parse for Property {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![@]>();
        let path: Path = input.parse()?;

        let key: Ident = input.parse()?;
        input.parse::<Token![:]>().expect("Expected ':' after property key");
        let ty_hint = input.parse::<Ident>()?;
        input.parse::<Token![=]>()?;
        let value: Expr = input.parse()?;

        Ok(Property { key, value,path, ty_hint })
    }
}

///Handles parsing for manual, more flexible archetype declarations, also derives InstanceReflect
pub struct ArchetypeExportParse {
    pub(crate) name: Ident,
    pub(crate) components: Vec<Path>,
    pub(crate)properties: Vec<Property>,
    pub(crate) readonly : Vec<Ident>
}
impl Parse for ArchetypeExportParse {
    fn parse(input:  ParseStream) -> Result<Self> {
        let mut name = None;
        let mut components :Vec<Path> = Vec::new();
        let mut properties : Vec<Property> = Vec::new();
        let mut readonly : Vec<Ident> = Vec::new();

        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;

            match ident.to_string().as_str() {
                "name" => {
                    input.parse::<Token![=]>()?;
                    name = Some(input.parse()?);
                }

                "components" => {
                    input.parse::<Token![=]>()?;
                    let content;
                    bracketed!(content in input);
                    
                    components = content.parse_terminated(Path::parse, Token![,])?.iter().map(Path::clone).collect();

                }

                "properties" => {
                    input.parse::<Token![=]>()?;
                    let content;
                    bracketed!(content in input);

                    let raw = content.parse_terminated(Property::parse, Token![,])?;

                    properties = raw.iter().map(|prop| { prop.clone() }).collect();
                }

                "readonly" => {
                    input.parse::<Token![=]>()?;
                    let content;
                    bracketed!(content in input);
                    
                    readonly = content.parse_terminated(Ident::parse, Token![,])?.iter().map(Ident::clone).collect();
                }

                _ => return Err(syn::Error::new(ident.span(), "Unknown field")),
            }

            // optional comma
            let _ = input.parse::<Token![,]>();
        }

        Ok(ArchetypeExportParse {
            name: name.ok_or_else(|| input.error("missing name"))?,
            components,
            properties,
            readonly,
        })
    }
}


pub struct ArchetypeParser {
    pub components: Vec<Vec<Path>>,
    pub names: Vec<Ident>,
}

pub fn field_names(paths: &Vec<Path>) -> Vec<Ident> {
    paths
        .iter()
        .map(|c| {
            let last = c.segments.last().expect("Component path cannot be empty").to_token_stream().to_string().to_lowercase();
            format_ident!("{}", last)
        })
        .collect()
}
pub fn archetype_name(ident: &Ident) -> Ident {
    format_ident!("{}", ident)
}

pub fn bundle_name(ident: &Ident) -> Ident {
    format_ident!("{}Bundle", ident)
}

impl ArchetypeParser {

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn reg_name(&self, i: usize) -> Ident {
        format_ident!(
            "{}DESCRIPTOR_REG",
            self.names[i].to_string().to_uppercase()
        )
    }
}

impl Parse for ArchetypeParser {

    fn parse(input: ParseStream) -> Result<Self> {
        let mut components = Vec::new();
        let mut names = Vec::new();

        while !input.is_empty() {

            let name: Ident = input.parse()?;
            names.push(name.clone());
            
            if input.peek(Token![,]) {
                
                components.push(vec![]);
                input.parse::<Token![,]>()?;
               // panic!("archetype with no components {}", name.clone().to_string().as_str() );
                continue
            }

            let content;
            bracketed!(content in input);

            let mut comp_list = Vec::new();

            while !content.is_empty() {
                comp_list.push(content.parse()?);

                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }

            components.push(comp_list);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        
        Ok(ArchetypeParser { components, names })
    }
}