use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::{Ident, ItemFn, Token, FnArg};

pub struct CustomFunctionAttributes {
    pub validations: Vec<Validation>,
}

pub struct Validation {
    name: Ident,
    args: Vec<FnArg>,
}

impl Parse for CustomFunctionAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<ItemFn, Token![,]>::parse_terminated(input)?;

        Ok(CustomFunctionAttributes {
            validations: vars
                .iter()
                .map(|v| {
                    let name = v.sig.ident.clone();
                    Validation {
                        name,
                        args: v
                            .sig
                            .inputs
                            .iter()
                            .map(|a| a.clone())
                            .collect(),
                    }
                })
                .collect(),
        })
    }
}

pub fn generate_validation_code(raw_validations: Vec<Validation>) -> proc_macro2::TokenStream {
    // let mut tokens = TokenStream::new();
    //
    // let mut validations: Vec<TokenStream> = Vec::new();
    //
    // for (index, validation) in raw_validations.iter().enumerate() {
    //     let name = &validation.name;
    //     let args = &validation.args;
    //
    //     let validation_code = if args.is_empty() {
    //         // Handle cases like: validate1
    //         quote! {
    //             #name()
    //         }
    //     } else {
    //         // Handle cases like: validate2(arg1, arg2)
    //         quote! {
    //             #name(#(#args),*)
    //         }
    //     };
    //
    //     // Add `.and` only if it's not the last validation
    //     if index < validations.len() - 1 {
    //         tokens.extend(quote! {
    //             #validation_code.and
    //         });
    //     } else {
    //         // If it's the last validation, remove the trailing `.and`
    //         tokens.extend(quote! {
    //             #validation_code
    //         });
    //     }
    // }
    //
    // // Combine all validations using `.and`
    // let combined_validations = quote! {
    //     #(#validations)*
    // };
    //
    // tokens.extend(combined_validations);
    //
    // tokens
    let mut tokens = TokenStream::new();

    let len = raw_validations.len();
    let validations: Vec<TokenStream> = raw_validations
        .iter()
        .enumerate()
        .map(|(i, validation)| {
            let name = &validation.name;
            let args = &validation.args;

            let a = if args.is_empty() {
                // Handle cases like: validate1
                quote! {
                    #name()
                }
            } else {
                // Handle cases like: validate2(arg1, arg2)
                quote! {
                    #name(#(#args),*)
                }
            };

            if i < len && i > 0 {
                return quote! {
                  .and(#a)
                };
            }
            a
        })
        .collect();

    // Combine all validations using `.and`
    let combined_validations = quote! {
        #(#validations)*
    };

    tokens.extend(combined_validations);

    tokens
}
