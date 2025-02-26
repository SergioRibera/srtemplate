use std::str::FromStr;

use proc_macro2::{TokenStream, TokenTree};
use quote::quote;

use venial::{parse_item, Attribute, AttributeValue, Error, Fields, Item, Struct};

use crate::TextCase;

pub fn derive(input: TokenStream) -> Result<TokenStream, Error> {
    let type_decl = parse_item(input);

    let res = match &type_decl {
        Ok(Item::Struct(struct_decl)) => derive_struct(struct_decl)?,
        // Ok(Item::Enum(enum_decl)) => derive_enum(enum_decl)?,
        _ => {
            return Err(Error::new(
                "currently only structs are supported by this derive",
            ))
        }
    };

    Ok(res)
}

fn parse_template_attribute(attr: &Attribute) -> Result<Vec<(String, String)>, Error> {
    let mut attributes = Vec::new();

    if !attr
        .get_single_path_segment()
        .is_some_and(|n| n.to_string().as_str() == "template")
    {
        return Ok(attributes);
    }

    match &attr.value {
        AttributeValue::Group(_span, tokens) => {
            let mut current_key = String::new();

            for token in tokens {
                match token {
                    TokenTree::Ident(ident) => {
                        let ident_str = ident.to_string();
                        if ident_str == "ignore" {
                            attributes.push(("ignore".to_string(), "true".to_string()));
                            continue;
                        }
                        current_key = ident_str;
                    }
                    TokenTree::Punct(p) if p.as_char() == '=' => {}
                    TokenTree::Literal(lit) => {
                        if !current_key.is_empty() {
                            let current_value = lit.to_string().trim_matches('"').to_string();
                            attributes.push((current_key.clone(), current_value));
                            current_key.clear();
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => return Err(Error::new("Expected group attribute")),
    }

    Ok(attributes)
}

fn derive_struct(struct_decl: &Struct) -> Result<TokenStream, Error> {
    let name_ident = &struct_decl.name;
    let mut struct_case = TextCase::Pascal;
    let mut field_case = Some(TextCase::Snake);

    for attr in struct_decl.attributes.iter() {
        let attrs = parse_template_attribute(attr)?;
        for (key, value) in attrs {
            match key.as_str() {
                "rename" => {
                    struct_case = TextCase::from_str(&value).map_err(|e| Error::new(e))?;
                }
                "rename_fields" => {
                    field_case = Some(TextCase::from_str(&value).map_err(|e| Error::new(e))?);
                }
                _ => {}
            }
        }
    }

    let impl_generics = &struct_decl.generic_params;
    let bounded_where_clause =
        struct_decl.create_derive_where_clause(quote!(miniserde::Deserialize));

    let fields = match &struct_decl.fields {
        Fields::Named(fields) => {
            let mut normalized_fields = Vec::new();

            for (field, _) in fields.fields.iter() {
                let mut ignore = false;
                let mut field_name = field.name.to_string();

                for attr in field.attributes.iter() {
                    let attrs = parse_template_attribute(attr)?;
                    for (key, value) in attrs {
                        match key.as_str() {
                            "ignore" => {
                                ignore = true;
                            }
                            "rename" => {
                                let case = TextCase::from_str(&value).map_err(|e| Error::new(e))?;
                                field_name = case.convert(&field_name);
                            }
                            _ => {}
                        }
                    }
                }

                if !ignore {
                    if let Some(case) = field_case {
                        field_name = case.convert(&field_name);
                    }

                    let name_ident = name_ident.to_string();
                    let name_ident = struct_case.convert(&name_ident);
                    let name = format!("{name_ident}.{field_name}");
                    let field = &field.name;

                    normalized_fields.push(quote! {
                        (
                            #name.into(),
                            self.#field.to_string()
                        )
                    });
                }
            }
            normalized_fields
        }
        Fields::Unit => return Err(Error::new("Unit structs are not supported")),
        Fields::Tuple(_) => return Err(Error::new("Tuple structs are not supported")),
    };

    Ok(quote! {
        impl<'variable, #impl_generics> srtemplate::Variable<'variable> for #name_ident<#impl_generics>
        #bounded_where_clause {
            fn variables(&self) -> impl Iterator<Item = (std::borrow::Cow<'variable, str>, String)> {
                [ #(#fields),* ].into_iter()
            }
        }
    })
}
