use attr::{generate_validation_code, CustomFunctionAttributes};
use function::FunctionComponent;
use quote::quote;
use syn::parse_macro_input;

extern crate proc_macro;

mod attr;
mod function;

#[proc_macro_attribute]
pub fn custom_function(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Analiza la función y sus atributos
    let input_fn = parse_macro_input!(input as FunctionComponent);
    let parsed_attr = parse_macro_input!(attr as CustomFunctionAttributes);

    let vis = &input_fn.vis;
    let function_name = &input_fn.name;
    let attrs = &input_fn.props_types;
    let function_block = &input_fn.block;
    //     .stmts
    //     .iter()
    //     .map(|s| s.to_token_stream())
    //     .collect::<Vec<_>>();
    //
    // // Obtener los nombres de las funciones de validación
    // let validation_names: Vec<String> = input_fn
    //     .attrs
    //     .iter()
    //     .filter_map(|attr| {
    //         if attr.path().segments.len() == 2 && attr.path().segments[1].ident == "custom_function"
    //         {
    //             attr.pound_token.span.source_text()
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();
    let validations = generate_validation_code(parsed_attr.validations);

    // Expande la función original con el código de validación
    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #vis struct #function_name;
        #[allow(non_camel_case_types)]
        impl CustomError for #function_name {
            type Args = (#(#attrs),*);
          fn validation(&self, args: Self::Args) -> Result<(), FunctionError> {
            #validations
          }

          fn call(&self, _args: Self::Args) -> Result<(), FunctionError>
              #function_block
        }
    };

    expanded.into()
}
