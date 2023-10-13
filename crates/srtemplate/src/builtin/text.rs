use crate::prelude::FuncResult;
use crate::template::validations;

pub fn to_lower(args: Vec<String>) -> FuncResult {
    validations::args_min_len(&args, 1)?;

    Ok(args
        .iter()
        .map(|a| a.to_lowercase())
        .collect::<Vec<_>>()
        .join(" "))
}

pub fn to_upper(args: Vec<String>) -> FuncResult {
    validations::args_min_len(&args, 1)?;

    Ok(args
        .iter()
        .map(|a| a.to_uppercase())
        .collect::<Vec<_>>()
        .join(" "))
}

pub fn trim(args: Vec<String>) -> FuncResult {
    validations::args_min_len(&args, 1)?;

    Ok(args.iter().map(|a| a.trim()).collect::<Vec<_>>().join(" "))
}
