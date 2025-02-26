use core::panic;

use function::gen_function;
use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use venial::{parse_item, Error, Item};
use quote::quote;

mod function;

#[proc_macro_attribute]
pub fn function(
    _: proc_macro::TokenStream,
    body: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let func = match parse_item(body.into()) {
        Ok(Item::Function(func)) => Ok(func),
        Err(e) => Err(e),
        Ok(_) => Err(Error::new("Just support functions")),
    };

    func.and_then(gen_function)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// This macro implements a way to convert structs into `SrTemplate`
/// instances.
///
/// The macro internally implements an anonymous function called
/// `template()` which converts the current struct into an instance
/// of `SrTemplate` using the struct fields and the instance values.
#[proc_macro_derive(Template)]
pub fn template(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let fields = if let Data::Struct(s) = input.data {
        if let Fields::Named(fields) = s.fields {
            fields.named
                .into_iter()
                .map(|f| f.ident.unwrap())
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    } else {
        panic!("This macro can only be applied to structs.")
    };

    let field_adds = fields
        .iter()
        .map(|field| {
            quote! {
                ctx.add_variable(stringify!(#field), &self.#field);
            }
        });

    quote! {
        impl #name {
            pub fn template(&self) -> srtemplate::SrTemplate {
                let mut ctx = srtempalte::SrTemplate::default();
                #(#field_adds)*

                ctx
            }
        }
    }
        .into()
}
