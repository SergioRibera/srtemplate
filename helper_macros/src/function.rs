use std::ops::Not;

use proc_macro2::{Ident, TokenStream};
use quote::quote_spanned;
use venial::{Error, Function as VenialFunc, TypeExpr};

pub fn gen_function(func: VenialFunc) -> Result<TokenStream, Error> {
    let func_name = &func.name;

    let vis = func.vis_marker.as_ref();

    let params = func
        .params
        .items()
        .filter_map(|param| match param {
            venial::FnParam::Receiver(receiver) => {
                Some(Err(Error::new_at_tokens(receiver, "self is not permitted")))
            }
            venial::FnParam::Typed(venial::FnTypedParam { name, ty, .. }) => {
                ty.tokens.is_empty().not().then_some(Ok((name, ty)))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let total_params = params.len();
    let decl = params
        .into_iter()
        .enumerate()
        .map(|(idx, (param, ty))| parse_param(idx, param, ty))
        .collect::<Vec<TokenStream>>();

    let func_body = func
        .body
        .as_ref()
        .ok_or_else(|| Error::new_at_span(func.span(), "Function should have body"))?;
    let func_body = quote_spanned! {func_body.span() => #func_body};

    Ok(quote_spanned! { func.span() =>
        #vis fn #func_name(args: &[String]) -> srtemplate::prelude::FuncResult {
            srtemplate::prelude::validations::args_min_len(args, #total_params)?;
            srtemplate::prelude::validations::args_max_len(args, #total_params)?;

            #(#decl)*

            #func_body
        }
    })
}

fn parse_param(idx: usize, param: &Ident, ty: &TypeExpr) -> TokenStream {
    quote_spanned! {ty.span() =>
        let #param = args[#idx].parse::<#ty>().map_err(|_| srtemplate::prelude::FromArgsError::ParseFailed(#idx))?;
    }
}
