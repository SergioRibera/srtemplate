use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, ItemFn};
use syn::{Attribute, Block, FnArg, Generics, Item, ReturnType, Type, Visibility};

#[derive(Clone)]
pub struct FunctionComponent {
    pub block: Box<Block>,
    pub props_types: Vec<Box<Type>>,
    pub generics: Generics,
    pub vis: Visibility,
    pub attrs: Vec<Attribute>,
    pub name: Ident,
}

impl Parse for FunctionComponent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parsed: Item = input.parse()?;

        let func = match parsed {
            Item::Fn(m) => m,

            item => {
                return Err(syn::Error::new_spanned(
                    item,
                    "`custom_function` attribute can only be applied to functions",
                ))
            }
        };

        let ItemFn {
            attrs,
            vis,
            sig,
            block,
        } = func;

        if sig.generics.lifetimes().next().is_some() {
            return Err(syn::Error::new_spanned(
                sig.generics,
                "function components can't have generic lifetime parameters",
            ));
        }

        if sig.asyncness.is_some() {
            return Err(syn::Error::new_spanned(
                sig.asyncness,
                "function components can't be async",
            ));
        }

        if sig.constness.is_some() {
            return Err(syn::Error::new_spanned(
                sig.constness,
                "const functions can't be function components",
            ));
        }

        if sig.abi.is_some() {
            return Err(syn::Error::new_spanned(
                sig.abi,
                "extern functions can't be function components",
            ));
        }

        if let ReturnType::Default = sig.output {
            return Err(syn::Error::new_spanned(
                sig,
                "function components must return `yew::Html` or `yew::HtmlResult`",
            ));
        }

        let mut props_types = Vec::new();
        for input_arg in &sig.inputs {
            match input_arg {
                FnArg::Typed(arg) => {
                    props_types.push(arg.ty.clone());
                }
                FnArg::Receiver(_) => {
                    return Err(syn::Error::new_spanned(
                        input_arg,
                        "function components can't accept a receiver",
                    ));
                }
            }
        }

        println!(
            "{:?}",
            props_types
                .iter()
                .map(|s| s.to_token_stream().to_string())
                .collect::<Vec<_>>()
        );

        Ok(Self {
            props_types,
            block,
            generics: sig.generics,
            vis,
            attrs,
            name: sig.ident,
        })
    }
}
