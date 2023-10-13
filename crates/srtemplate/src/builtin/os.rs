use crate::prelude::{FuncResult, FunctionError};
use crate::template::validations;

pub fn env(args: Vec<String>) -> FuncResult {
    validations::args_min_len(&args, 1)?;

    Ok(args
        .iter()
        .map(|a| {
            std::env::var(a)
                .map_err(|_| FunctionError::InvalidArgument(a.clone()))
                .unwrap()
        })
        .collect::<Vec<String>>()
        .join(" "))
}
