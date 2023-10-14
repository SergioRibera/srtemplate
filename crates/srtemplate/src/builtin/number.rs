#![allow(unused)]

use concat_idents::concat_idents;
use std::ops::{Add, Sub, Mul, Div};

use crate::function::FuncResult;
use crate::prelude::validations;

macro_rules! gen_math_fn {
    ($name: ident, $( $t: ty ),* ) => {
        $(
            concat_idents!(fn_name = $name, _, $t {
                pub fn fn_name(args: Vec<String>) -> FuncResult {
                    for arg in &args {
                        validations::arg_type::<$t>(arg.clone())?;
                    }
                    Ok(args.iter()
                        .map(|a| a.parse::<$t>().unwrap_or_default())
                        .fold($t::default(), |acc, x| acc.$name(x))
                        .to_string())
                }
            });
        )*
    };
}

gen_math_fn!(add, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(sub, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(mul, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(div, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
