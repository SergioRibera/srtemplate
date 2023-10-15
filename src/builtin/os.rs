use crate::prelude::{FuncResult, FunctionError};
use crate::template::validations;

pub fn env(args: &[String]) -> FuncResult {
    validations::args_min_len(args, 1)?;

    Ok(args
        .iter()
        .map(|a| std::env::var(a).map_err(|_| FunctionError::InvalidArgument(a.clone())))
        .collect::<Result<Vec<String>, FunctionError>>()?
        .join(" "))
}
