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
    let name_ident = struct_decl.name.clone();
    let mut name_struct = name_ident.to_string();
    let mut struct_case = None;
    let mut field_case = Some(TextCase::Snake);

    for attr in struct_decl.attributes.iter() {
        let attrs = parse_template_attribute(attr)?;
        for (key, value) in attrs {
            match key.as_str() {
                "rename" | "alias" => {
                    name_struct = value;
                }
                "case" => {
                    struct_case = Some(TextCase::from_str(&value).map_err(|e| Error::new(e))?);
                }
                "case_fields" => {
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
                            "case" => {
                                let case = TextCase::from_str(&value).map_err(|e| Error::new(e))?;
                                field_name = case.convert(&field_name);
                            }
                            "rename" | "alias" => {
                                field_name = value;
                            }
                            _ => {}
                        }
                    }
                }

                if !ignore {
                    if let Some(case) = field_case {
                        field_name = case.convert(&field_name);
                    }

                    let name_ident = if name_struct.is_empty() {
                        let name_ident = name_ident.to_string();
                        struct_case
                            .map(|s| s.convert(&name_ident))
                            .unwrap_or(name_ident)
                    } else {
                        struct_case
                            .map(|s| s.convert(&name_struct))
                            .unwrap_or(name_struct.clone())
                    };
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
        Fields::Unit => {
            let name_ident = if name_struct.is_empty() {
                let name_ident = name_ident.to_string();
                struct_case
                    .map(|s| s.convert(&name_ident))
                    .unwrap_or(name_ident)
            } else {
                struct_case
                    .map(|s| s.convert(&name_struct))
                    .unwrap_or(name_struct)
            };

            vec![quote! {
                (
                    #name_ident.into(),
                    self.to_string()
                )
            }]
        }
        Fields::Tuple(tuple) => tuple
            .fields
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let i = proc_macro2::Literal::usize_unsuffixed(i);
                let name_ident = if name_struct.is_empty() {
                    let name_ident = name_ident.to_string();
                    struct_case
                        .map(|s| s.convert(&name_ident))
                        .unwrap_or(name_ident)
                } else {
                    struct_case
                        .map(|s| s.convert(&name_struct))
                        .unwrap_or(name_struct.clone())
                };
                let name = format!("{name_ident}.{i}");
                quote! {
                    (
                        #name.into(),
                        self.#i.to_string()
                    )
                }
            })
            .collect(),
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
