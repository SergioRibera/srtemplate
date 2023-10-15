#![allow(unused)]

use paste::paste;
use std::ops::{Add, Div, Mul, Sub};

use crate::function::FuncResult;
use crate::prelude::validations;

macro_rules! gen_math_fn {
    ($name: ident, $( $t: ty ),* ) => {
        $(
            paste! {
                pub fn [<$name _ $t>](args: &[String]) -> FuncResult {
                    for arg in args {
                        validations::arg_type::<$t>(arg.clone())?;
                    }
                    Ok(args.iter()
                        .map(|a| a.parse::<$t>().unwrap_or_default())
                        .fold($t::default(), |acc, x| acc.$name(x))
                        .to_string())
                }
            }
        )*
    };
}

gen_math_fn!(add, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(sub, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(mul, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
gen_math_fn!(div, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

#[macro_export]
macro_rules! gen_math_use {
    ($tmp:ident) => {
        gen_math_use!($tmp, add, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
        gen_math_use!($tmp, sub, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
        gen_math_use!($tmp, mul, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
        gen_math_use!($tmp, div, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);
    };

    ($tmp: ident, $name: ident, $( $t: ty ),* ) => {
        $(
            paste! {
                $tmp.add_function(stringify!([<$name _ $t>]), builtin::[<$name _ $t>]);
            }
        )*
    };
}
