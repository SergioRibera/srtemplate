use function::gen_function;
use venial::{parse_item, Error, Item};

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
